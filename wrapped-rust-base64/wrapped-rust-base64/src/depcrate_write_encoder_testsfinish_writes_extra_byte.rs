// Generated macro for finish_writes_extra_byte (function)
macro_rules! Depcrate_write_encoder_testsfinish_writes_extra_byte {
() => {
// Module: crate::write::encoder_tests
// Provides: {"finish_writes_extra_byte"}
// Dependencies: {}
# [test] fn finish_writes_extra_byte () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; assert_eq ! (6 , enc . write (b"abcdef") . unwrap ()) ; assert_eq ! (1 , enc . write (b"g") . unwrap ()) ; let _ = enc . finish () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcdefg") . as_bytes ()) ; }
};
}
