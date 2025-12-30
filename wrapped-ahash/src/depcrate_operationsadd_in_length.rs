// Generated macro for add_in_length (function)
macro_rules! Depcrate_operationsadd_in_length {
() => {
// Module: crate::operations
// Provides: {"add_in_length"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] pub (crate) fn add_in_length (enc : & mut u128 , len : u64) { # [cfg (all (target_arch = "x86_64" , target_feature = "sse2" , not (miri)))] { # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: * ; unsafe { let enc = enc as * mut u128 ; let len = _mm_cvtsi64_si128 (len as i64) ; let data = _mm_loadu_si128 (enc . cast ()) ; let sum = _mm_add_epi64 (data , len) ; _mm_storeu_si128 (enc . cast () , sum) ; } } # [cfg (not (all (target_arch = "x86_64" , target_feature = "sse2" , not (miri))))] { let mut t : [u64 ; 2] = enc . convert () ; t [0] = t [0] . wrapping_add (len) ; * enc = t . convert () ; } }
};
}
