// Generated macro for write_partial_then_enough_to_complete_chunk_and_another_chunk_encodes_complete_chunks (function)
macro_rules! Depcrate_write_encoder_testswrite_partial_then_enough_to_complete_chunk_and_another_chunk_encodes_complete_chunks {
() => {
// Module: crate::write::encoder_tests
// Provides: {"write_partial_then_enough_to_complete_chunk_and_another_chunk_encodes_complete_chunks"}
// Dependencies: {}
# [test] fn write_partial_then_enough_to_complete_chunk_and_another_chunk_encodes_complete_chunks () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & NO_PAD_ENGINE) ; assert_eq ! (1 , enc . write (b"a") . unwrap ()) ; assert_eq ! (5 , enc . write (b"bcdef") . unwrap ()) ; let _ = enc . finish () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , NO_PAD_ENGINE . encode ("abcdef") . as_bytes ()) ; assert_eq ! (8 , c . get_ref () . len ()) ; }
};
}
