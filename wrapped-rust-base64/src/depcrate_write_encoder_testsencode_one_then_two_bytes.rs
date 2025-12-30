// Generated macro for encode_one_then_two_bytes (function)
macro_rules! Depcrate_write_encoder_testsencode_one_then_two_bytes {
() => {
// Module: crate::write::encoder_tests
// Provides: {"encode_one_then_two_bytes"}
// Dependencies: {}
# [test] fn encode_one_then_two_bytes () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; let sz = enc . write (b"a") . unwrap () ; assert_eq ! (sz , 1) ; let sz = enc . write (b"bc") . unwrap () ; assert_eq ! (sz , 2) ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abc") . as_bytes ()) ; }
};
}
