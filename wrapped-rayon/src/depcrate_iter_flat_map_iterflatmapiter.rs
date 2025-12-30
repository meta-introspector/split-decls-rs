// Generated macro for FlatMapIter (struct)
macro_rules! Depcrate_iter_flat_map_iterFlatMapIter {
() => {
// Module: crate::iter::flat_map_iter
// Provides: {"FlatMapIter"}
// Dependencies: {}
# [doc = " `FlatMapIter` maps each element to a serial iterator, then flattens these iterators together."] # [doc = " This struct is created by the [`flat_map_iter()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`flat_map_iter()`]: ParallelIterator::flat_map_iter()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct FlatMapIter < I , F > { base : I , map_op : F , }
};
}
