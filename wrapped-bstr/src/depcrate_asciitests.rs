// Generated macro for tests (module)
macro_rules! Depcrate_asciitests {
() => {
// Module: crate::ascii
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn positive_fallback_forward () { for i in 0 .. 517 { let s = "a" . repeat (i) ; assert_eq ! (i , first_non_ascii_byte_fallback (s . as_bytes ()) , "i: {:?}, len: {:?}, s: {:?}" , i , s . len () , s) ; } } # [test] # [cfg (target_arch = "x86_64")] # [cfg (not (miri))] fn positive_sse2_forward () { for i in 0 .. 517 { let b = "a" . repeat (i) . into_bytes () ; assert_eq ! (b . len () , first_non_ascii_byte_sse2 (& b)) ; } } # [test] # [cfg (not (miri))] fn negative_fallback_forward () { for i in 0 .. 517 { for align in 0 .. 65 { let mut s = "a" . repeat (i) ; s . push_str ("☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃") ; let s = s . get (align ..) . unwrap_or ("") ; assert_eq ! (i . saturating_sub (align) , first_non_ascii_byte_fallback (s . as_bytes ()) , "i: {:?}, align: {:?}, len: {:?}, s: {:?}" , i , align , s . len () , s) ; } } } # [test] # [cfg (target_arch = "x86_64")] # [cfg (not (miri))] fn negative_sse2_forward () { for i in 0 .. 517 { for align in 0 .. 65 { let mut s = "a" . repeat (i) ; s . push_str ("☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃") ; let s = s . get (align ..) . unwrap_or ("") ; assert_eq ! (i . saturating_sub (align) , first_non_ascii_byte_sse2 (s . as_bytes ()) , "i: {:?}, align: {:?}, len: {:?}, s: {:?}" , i , align , s . len () , s) ; } } } }
};
}
