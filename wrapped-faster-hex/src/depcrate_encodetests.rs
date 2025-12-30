// Generated macro for tests (module)
macro_rules! Depcrate_encodetests {
() => {
// Module: crate::encode
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: encode :: { hex_encode , hex_encode_custom_case_fallback } ; use crate :: hex_encode_fallback ; use core :: str ; use proptest :: proptest ; fn _test_encode_fallback (s : & String , upper_case : bool) { let mut buffer = vec ! [0 ; s . as_bytes () . len () * 2] ; hex_encode_custom_case_fallback (s . as_bytes () , & mut buffer , upper_case) ; let encode = unsafe { str :: from_utf8_unchecked (& buffer [.. s . as_bytes () . len () * 2]) } ; if upper_case { assert_eq ! (encode , hex :: encode_upper (s)) ; } else { assert_eq ! (encode , hex :: encode (s)) ; } } proptest ! { # [test] fn test_encode_fallback (ref s in ".*") { _test_encode_fallback (s , true) ; _test_encode_fallback (s , false) ; } } # [test] fn test_encode_zero_length_src_should_be_ok () { let src = b"" ; let mut dst = [0u8 ; 10] ; assert ! (hex_encode (src , & mut dst) . is_ok ()) ; hex_encode_fallback (src , & mut dst) ; } }
};
}
