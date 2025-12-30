// Generated macro for Permutations (struct)
macro_rules! Depcrate_permutationsPermutations {
() => {
// Module: crate::permutations
// Provides: {"Permutations"}
// Dependencies: {}
# [doc = " An iterator adaptor that iterates through all the `k`-permutations of the"] # [doc = " elements from an iterator."] # [doc = ""] # [doc = " See [`.permutations()`](crate::Itertools::permutations) for"] # [doc = " more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Permutations < I : Iterator > { vals : LazyBuffer < I > , state : PermutationState , }
};
}
