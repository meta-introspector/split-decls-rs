// Generated macro for set_bytes (function)
macro_rules! Depcrate_mem_implsset_bytes {
() => {
// Module: crate::mem::impls
// Provides: {"set_bytes"}
// Dependencies: {}
# [inline (always)] pub unsafe fn set_bytes (mut s : * mut u8 , c : u8 , mut n : usize) { # [inline (always)] pub unsafe fn set_bytes_bytes (mut s : * mut u8 , c : u8 , n : usize) { let end = s . wrapping_add (n) ; while s < end { * s = c ; s = s . wrapping_add (1) ; } } # [inline (always)] pub unsafe fn set_bytes_words (s : * mut u8 , c : u8 , n : usize) { let mut broadcast = c as usize ; let mut bits = 8 ; while bits < WORD_SIZE * 8 { broadcast |= broadcast << bits ; bits *= 2 ; } let mut s_usize = s as * mut usize ; let end = s . wrapping_add (n) as * mut usize ; while s_usize < end { * s_usize = broadcast ; s_usize = s_usize . wrapping_add (1) ; } } if likely (n >= WORD_COPY_THRESHOLD) { let misalignment = (s as usize) . wrapping_neg () & WORD_MASK ; set_bytes_bytes (s , c , misalignment) ; s = s . wrapping_add (misalignment) ; n -= misalignment ; let n_words = n & ! WORD_MASK ; set_bytes_words (s , c , n_words) ; s = s . wrapping_add (n_words) ; n -= n_words ; } set_bytes_bytes (s , c , n) ; }
};
}
