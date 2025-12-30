// Generated macro for write_partial_chunk_encodes_partial_chunk (function)
macro_rules! Depcrate_write_encoder_testswrite_partial_chunk_encodes_partial_chunk {
() => {
// Module: crate::write::encoder_tests
// Provides: {"write_partial_chunk_encodes_partial_chunk"}
// Dependencies: {}
# [test] fn write_partial_chunk_encodes_partial_chunk () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & NO_PAD_ENGINE) ; assert_eq ! (2 , enc . write (b"ab") . unwrap ()) ; let _ = enc . finish () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , NO_PAD_ENGINE . encode ("ab") . as_bytes ()) ; assert_eq ! (3 , c . get_ref () . len ()) ; }
};
}
