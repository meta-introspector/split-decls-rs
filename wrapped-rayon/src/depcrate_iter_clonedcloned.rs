// Generated macro for Cloned (struct)
macro_rules! Depcrate_iter_clonedCloned {
() => {
// Module: crate::iter::cloned
// Provides: {"Cloned"}
// Dependencies: {}
# [doc = " `Cloned` is an iterator that clones the elements of an underlying iterator."] # [doc = ""] # [doc = " This struct is created by the [`cloned()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`cloned()`]: ParallelIterator::cloned()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct Cloned < I > { base : I , }
};
}
