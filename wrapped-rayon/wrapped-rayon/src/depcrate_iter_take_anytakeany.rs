// Generated macro for TakeAny (struct)
macro_rules! Depcrate_iter_take_anyTakeAny {
() => {
// Module: crate::iter::take_any
// Provides: {"TakeAny"}
// Dependencies: {}
# [doc = " `TakeAny` is an iterator that iterates over `n` elements from anywhere in `I`."] # [doc = " This struct is created by the [`take_any()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`take_any()`]: ParallelIterator::take_any()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct TakeAny < I > { base : I , count : usize , }
};
}
