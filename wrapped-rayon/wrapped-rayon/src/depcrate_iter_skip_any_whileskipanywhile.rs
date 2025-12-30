// Generated macro for SkipAnyWhile (struct)
macro_rules! Depcrate_iter_skip_any_whileSkipAnyWhile {
() => {
// Module: crate::iter::skip_any_while
// Provides: {"SkipAnyWhile"}
// Dependencies: {}
# [doc = " `SkipAnyWhile` is an iterator that skips over elements from anywhere in `I`"] # [doc = " until the callback returns `false`."] # [doc = " This struct is created by the [`skip_any_while()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`skip_any_while()`]: ParallelIterator::skip_any_while()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct SkipAnyWhile < I , P > { base : I , predicate : P , }
};
}
