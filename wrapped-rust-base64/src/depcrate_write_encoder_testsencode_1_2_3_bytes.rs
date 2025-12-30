// Generated macro for encode_1_2_3_bytes (function)
macro_rules! Depcrate_write_encoder_testsencode_1_2_3_bytes {
() => {
// Module: crate::write::encoder_tests
// Provides: {"encode_1_2_3_bytes"}
// Dependencies: {}
# [test] fn encode_1_2_3_bytes () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; let sz = enc . write (b"a") . unwrap () ; assert_eq ! (sz , 1) ; let sz = enc . write (b"bc") . unwrap () ; assert_eq ! (sz , 2) ; let sz = enc . write (b"def") . unwrap () ; assert_eq ! (sz , 3) ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcdef") . as_bytes ()) ; }
};
}
