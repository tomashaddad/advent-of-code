use std::ops::RangeInclusive;

use crate::problem::Day;

pub struct Code;

trait Containment {
    fn is_subset(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl Containment for RangeInclusive<u32> {
    fn is_subset(&self, other: &Self) -> bool {
        self.start() >= other.start() && self.end() <= other.end()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }
}

fn to_range(s: &str) -> RangeInclusive<u32> {
    s.split_once('-')
        .map(|(a, b)| RangeInclusive::new(a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unwrap()
}

impl Day for Code {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                line.split_once(',')
                    .map(|(first, second)| (to_range(first), to_range(second)))
            })
            .filter(|allocation| {
                let (first, second) = allocation.clone().unwrap();
                first.is_subset(&second) || second.is_subset(&first)
            })
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(|line| {
                line.split_once(',')
                    .map(|(first, second)| (to_range(first), to_range(second)))
            })
            .filter(|allocation| {
                let (first, second) = allocation.clone().unwrap();
                first.overlaps(&second) || second.overlaps(&first)
            })
            .count()
            .to_string()
    }
}
