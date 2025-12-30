// Generated macro for interleave_shortest (function)
macro_rules! Depcrate_adaptorsinterleave_shortest {
() => {
// Module: crate::adaptors
// Provides: {"interleave_shortest"}
// Dependencies: {}
# [doc = " Create a new `InterleaveShortest` iterator."] pub fn interleave_shortest < I , J > (i : I , j : J) -> InterleaveShortest < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { InterleaveShortest { i , j , next_coming_from_j : false , } }
};
}
