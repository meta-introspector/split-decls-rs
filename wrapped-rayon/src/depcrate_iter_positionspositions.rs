// Generated macro for Positions (struct)
macro_rules! Depcrate_iter_positionsPositions {
() => {
// Module: crate::iter::positions
// Provides: {"Positions"}
// Dependencies: {}
# [doc = " `Positions` takes a predicate `predicate` and filters out elements that match,"] # [doc = " yielding their indices."] # [doc = ""] # [doc = " This struct is created by the [`positions()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`positions()`]: IndexedParallelIterator::positions()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Positions < I , P > { base : I , predicate : P , }
};
}
