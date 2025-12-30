// Generated macro for stubs (module)
macro_rules! Depcratestubs {
() => {
// Module: crate
// Provides: {"stubs"}
// Dependencies: {}
# [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] # [allow (clippy :: arithmetic_side_effects)] pub mod stubs { use super :: is_nonoverlapping ; # [doc = " # Safety"] pub unsafe fn sol_memcpy (dst : * mut u8 , src : * const u8 , n : usize) { assert ! (is_nonoverlapping (src as usize , n , dst as usize , n) , "memcpy does not support overlapping regions") ; core :: ptr :: copy_nonoverlapping (src , dst , n) ; } # [doc = " # Safety"] pub unsafe fn sol_memmove (dst : * mut u8 , src : * const u8 , n : usize) { core :: ptr :: copy (src , dst , n) ; } # [doc = " # Safety"] pub unsafe fn sol_memcmp (s1 : * const u8 , s2 : * const u8 , n : usize , result : * mut i32) { let mut i = 0 ; while i < n { let a = * s1 . add (i) ; let b = * s2 . add (i) ; if a != b { * result = a as i32 - b as i32 ; return ; } i += 1 ; } * result = 0 } # [doc = " # Safety"] pub unsafe fn sol_memset (s : * mut u8 , c : u8 , n : usize) { let s = core :: slice :: from_raw_parts_mut (s , n) ; for val in s . iter_mut () . take (n) { * val = c ; } } }
};
}
