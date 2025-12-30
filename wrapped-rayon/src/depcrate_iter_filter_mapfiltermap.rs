// Generated macro for FilterMap (struct)
macro_rules! Depcrate_iter_filter_mapFilterMap {
() => {
// Module: crate::iter::filter_map
// Provides: {"FilterMap"}
// Dependencies: {}
# [doc = " `FilterMap` creates an iterator that uses `filter_op` to both filter and map elements."] # [doc = " This struct is created by the [`filter_map()`] method on [`ParallelIterator`]."] # [doc = ""] # [doc = " [`filter_map()`]: ParallelIterator::filter_map()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FilterMap < I , P > { base : I , filter_op : P , }
};
}
