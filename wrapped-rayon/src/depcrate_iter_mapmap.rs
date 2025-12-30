// Generated macro for Map (struct)
macro_rules! Depcrate_iter_mapMap {
() => {
// Module: crate::iter::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " `Map` is an iterator that transforms the elements of an underlying iterator."] # [doc = ""] # [doc = " This struct is created by the [`map()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`map()`]: ParallelIterator::map()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Map < I , F > { base : I , map_op : F , }
};
}
