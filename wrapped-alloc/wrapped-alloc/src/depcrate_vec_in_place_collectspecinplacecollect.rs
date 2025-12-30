// Generated macro for SpecInPlaceCollect (trait)
macro_rules! Depcrate_vec_in_place_collectSpecInPlaceCollect {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"SpecInPlaceCollect"}
// Dependencies: {}
# [doc = " Helper trait to hold specialized implementations of the in-place iterate-collect loop"] trait SpecInPlaceCollect < T , I > : Iterator < Item = T > { # [doc = " Collects an iterator (`self`) into the destination buffer (`dst`) and returns the number of items"] # [doc = " collected. `end` is the last writable element of the allocation and used for bounds checks."] # [doc = ""] # [doc = " This method is specialized and one of its implementations makes use of"] # [doc = " `Iterator::__iterator_get_unchecked` calls with a `TrustedRandomAccessNoCoerce` bound"] # [doc = " on `I` which means the caller of this method must take the safety conditions"] # [doc = " of that trait into consideration."] unsafe fn collect_in_place (& mut self , dst : * mut T , end : * const T) -> usize ; }
};
}
