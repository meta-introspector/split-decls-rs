// Generated macro for needs_realloc (function)
macro_rules! Depcrate_vec_in_place_collectneeds_realloc {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"needs_realloc"}
// Dependencies: {}
const fn needs_realloc < SRC , DEST > (src_cap : usize , dst_cap : usize) -> bool { if const { align_of :: < SRC > () != align_of :: < DEST > () } { panic ! ("in_place_collectible() prevents this") ; } if const { let src_sz = size_of :: < SRC > () ; let dest_sz = size_of :: < DEST > () ; dest_sz != 0 && src_sz % dest_sz == 0 } { return false ; } src_cap > 0 && src_cap * size_of :: < SRC > () != dst_cap * size_of :: < DEST > () }
};
}
