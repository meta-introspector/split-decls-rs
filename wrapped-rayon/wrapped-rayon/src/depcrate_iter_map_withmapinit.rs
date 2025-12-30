// Generated macro for MapInit (struct)
macro_rules! Depcrate_iter_map_withMapInit {
() => {
// Module: crate::iter::map_with
// Provides: {"MapInit"}
// Dependencies: {}
# [doc = " `MapInit` is an iterator that transforms the elements of an underlying iterator."] # [doc = ""] # [doc = " This struct is created by the [`map_init()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`map_init()`]: ParallelIterator::map_init()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct MapInit < I , INIT , F > { base : I , init : INIT , map_op : F , }
};
}
