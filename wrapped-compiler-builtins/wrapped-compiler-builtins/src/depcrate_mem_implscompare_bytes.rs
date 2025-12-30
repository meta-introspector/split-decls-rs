// Generated macro for compare_bytes (function)
macro_rules! Depcrate_mem_implscompare_bytes {
() => {
// Module: crate::mem::impls
// Provides: {"compare_bytes"}
// Dependencies: {}
# [inline (always)] pub unsafe fn compare_bytes (s1 : * const u8 , s2 : * const u8 , n : usize) -> c_int { let mut i = 0 ; while i < n { let a = * s1 . wrapping_add (i) ; let b = * s2 . wrapping_add (i) ; if a != b { return c_int :: from (a) - c_int :: from (b) ; } i += 1 ; } 0 }
};
}
