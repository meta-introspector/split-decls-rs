// Generated macro for quarter_round (function)
macro_rules! Depcratequarter_round {
() => {
// Module: crate
// Provides: {"quarter_round"}
// Dependencies: {}
# [doc = " The ChaCha20 quarter round function"] # [doc = ""] # [doc = " We located this function in the root of the crate as we want it to be available"] # [doc = " for the soft backend and for xchacha."] # [allow (dead_code)] pub (crate) fn quarter_round (a : usize , b : usize , c : usize , d : usize , state : & mut [u32 ; STATE_WORDS] ,) { state [a] = state [a] . wrapping_add (state [b]) ; state [d] ^= state [a] ; state [d] = state [d] . rotate_left (16) ; state [c] = state [c] . wrapping_add (state [d]) ; state [b] ^= state [c] ; state [b] = state [b] . rotate_left (12) ; state [a] = state [a] . wrapping_add (state [b]) ; state [d] ^= state [a] ; state [d] = state [d] . rotate_left (8) ; state [c] = state [c] . wrapping_add (state [d]) ; state [b] ^= state [c] ; state [b] = state [b] . rotate_left (7) ; }
};
}
