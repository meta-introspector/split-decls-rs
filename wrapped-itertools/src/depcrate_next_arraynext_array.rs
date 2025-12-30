// Generated macro for next_array (function)
macro_rules! Depcrate_next_arraynext_array {
() => {
// Module: crate::next_array
// Provides: {"next_array"}
// Dependencies: {}
# [doc = " Equivalent to `it.next_array()`."] pub (crate) fn next_array < I , const N : usize > (it : & mut I) -> Option < [I :: Item ; N] > where I : Iterator , { let mut builder = ArrayBuilder :: new () ; for _ in 0 .. N { builder . push (it . next () ?) ; } builder . take () }
};
}
