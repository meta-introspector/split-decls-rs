// Generated macro for InterleaveShortest (struct)
macro_rules! Depcrate_adaptorsInterleaveShortest {
() => {
// Module: crate::adaptors
// Provides: {"InterleaveShortest"}
// Dependencies: {}
# [doc = " An iterator adaptor that alternates elements from the two iterators until"] # [doc = " one of them runs out."] # [doc = ""] # [doc = " This iterator is *fused*."] # [doc = ""] # [doc = " See [`.interleave_shortest()`](crate::Itertools::interleave_shortest)"] # [doc = " for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct InterleaveShortest < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { i : I , j : J , next_coming_from_j : bool , }
};
}
