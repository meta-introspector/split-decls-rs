// Generated macro for Flatten (struct)
macro_rules! Depcrate_iter_flattenFlatten {
() => {
// Module: crate::iter::flatten
// Provides: {"Flatten"}
// Dependencies: {}
# [doc = " `Flatten` turns each element to a parallel iterator, then flattens these iterators"] # [doc = " together. This struct is created by the [`flatten()`] method on [`ParallelIterator`]."] # [doc = ""] # [doc = " [`flatten()`]: ParallelIterator::flatten()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Flatten < I > { base : I , }
};
}
