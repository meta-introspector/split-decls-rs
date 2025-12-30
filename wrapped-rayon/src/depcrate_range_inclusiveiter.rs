// Generated macro for Iter (struct)
macro_rules! Depcrate_range_inclusiveIter {
() => {
// Module: crate::range_inclusive
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over an inclusive range, implemented for all integer types and `char`."] # [doc = ""] # [doc = " **Note:** The `zip` operation requires `IndexedParallelIterator`"] # [doc = " which is only implemented for `u8`, `i8`, `u16`, `i16`, and `char`."] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let p = (0..=25u16).into_par_iter()"] # [doc = "                   .zip(0..=25u16)"] # [doc = "                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)"] # [doc = "                   .map(|(x, y)| x * y)"] # [doc = "                   .sum::<u16>();"] # [doc = ""] # [doc = " let s = (0..=25u16).zip(0..=25u16)"] # [doc = "                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)"] # [doc = "                   .map(|(x, y)| x * y)"] # [doc = "                   .sum();"] # [doc = ""] # [doc = " assert_eq!(p, s);"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Iter < T > { range : RangeInclusive < T > , }
};
}
