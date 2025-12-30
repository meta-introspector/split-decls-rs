// Generated macro for SkipAny (struct)
macro_rules! Depcrate_iter_skip_anySkipAny {
() => {
// Module: crate::iter::skip_any
// Provides: {"SkipAny"}
// Dependencies: {}
# [doc = " `SkipAny` is an iterator that skips over `n` elements from anywhere in `I`."] # [doc = " This struct is created by the [`skip_any()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`skip_any()`]: ParallelIterator::skip_any()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct SkipAny < I > { base : I , count : usize , }
};
}
