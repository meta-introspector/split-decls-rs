// Generated macro for Iter (struct)
macro_rules! Depcrate_rangeIter {
() => {
// Module: crate::range
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Parallel iterator over a range, implemented for all integer types and `char`."] # [doc = ""] # [doc = " **Note:** The `zip` operation requires `IndexedParallelIterator`"] # [doc = " which is not implemented for `u64`, `i64`, `u128`, or `i128`."] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = ""] # [doc = " let p = (0..25usize).into_par_iter()"] # [doc = "                   .zip(0..25usize)"] # [doc = "                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)"] # [doc = "                   .map(|(x, y)| x * y)"] # [doc = "                   .sum::<usize>();"] # [doc = ""] # [doc = " let s = (0..25usize).zip(0..25)"] # [doc = "                   .filter(|&(x, y)| x % 5 == 0 || y % 5 == 0)"] # [doc = "                   .map(|(x, y)| x * y)"] # [doc = "                   .sum();"] # [doc = ""] # [doc = " assert_eq!(p, s);"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct Iter < T > { range : Range < T > , }
};
}
