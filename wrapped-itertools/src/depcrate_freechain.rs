// Generated macro for chain (function)
macro_rules! Depcrate_freechain {
() => {
// Module: crate::free
// Provides: {"chain"}
// Dependencies: {}
# [doc = " Takes two iterables and creates a new iterator over both in sequence."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::chain`]."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```"] # [doc = " use itertools::chain;"] # [doc = ""] # [doc = " let mut result: Vec<i32> = Vec::new();"] # [doc = ""] # [doc = " for element in chain(&[1, 2, 3], &[4]) {"] # [doc = "     result.push(*element);"] # [doc = " }"] # [doc = " assert_eq!(result, vec![1, 2, 3, 4]);"] # [doc = " ```"] pub fn chain < I , J > (i : I , j : J ,) -> iter :: Chain < < I as IntoIterator > :: IntoIter , < J as IntoIterator > :: IntoIter > where I : IntoIterator , J : IntoIterator < Item = I :: Item > , { i . into_iter () . chain (j) }
};
}
