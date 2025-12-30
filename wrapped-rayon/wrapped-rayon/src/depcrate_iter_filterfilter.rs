// Generated macro for Filter (struct)
macro_rules! Depcrate_iter_filterFilter {
() => {
// Module: crate::iter::filter
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " `Filter` takes a predicate `filter_op` and filters out elements that match."] # [doc = " This struct is created by the [`filter()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`filter()`]: ParallelIterator::filter()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Filter < I , P > { base : I , filter_op : P , }
};
}
