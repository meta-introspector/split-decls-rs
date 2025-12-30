// Generated macro for MinLen (struct)
macro_rules! Depcrate_iter_lenMinLen {
() => {
// Module: crate::iter::len
// Provides: {"MinLen"}
// Dependencies: {}
# [doc = " `MinLen` is an iterator that imposes a minimum length on iterator splits."] # [doc = " This struct is created by the [`with_min_len()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`with_min_len()`]: IndexedParallelIterator::with_min_len()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct MinLen < I > { base : I , min : usize , }
};
}
