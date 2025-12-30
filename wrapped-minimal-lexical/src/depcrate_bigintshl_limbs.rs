// Generated macro for shl_limbs (function)
macro_rules! Depcrate_bigintshl_limbs {
() => {
// Module: crate::bigint
// Provides: {"shl_limbs"}
// Dependencies: {}
# [doc = " Shift-left `n` limbs inside a buffer."] # [inline] pub fn shl_limbs (x : & mut VecType , n : usize) -> Option < () > { debug_assert ! (n != 0) ; if n + x . len () > x . capacity () { None } else if ! x . is_empty () { let len = n + x . len () ; unsafe { let src = x . as_ptr () ; let dst = x . as_mut_ptr () . add (n) ; ptr :: copy (src , dst , x . len ()) ; ptr :: write_bytes (x . as_mut_ptr () , 0 , n) ; x . set_len (len) ; } Some (()) } else { Some (()) } }
};
}
