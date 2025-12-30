// Generated macro for encode_with_padding (function)
macro_rules! Depcrate_write_encoder_testsencode_with_padding {
() => {
// Module: crate::write::encoder_tests
// Provides: {"encode_with_padding"}
// Dependencies: {}
# [test] fn encode_with_padding () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; enc . write_all (b"abcd") . unwrap () ; enc . flush () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcd") . as_bytes ()) ; }
};
}
