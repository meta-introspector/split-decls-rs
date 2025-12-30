// Generated macro for drop_calls_finish_for_you (function)
macro_rules! Depcrate_write_encoder_testsdrop_calls_finish_for_you {
() => {
// Module: crate::write::encoder_tests
// Provides: {"drop_calls_finish_for_you"}
// Dependencies: {}
# [test] fn drop_calls_finish_for_you () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & NO_PAD_ENGINE) ; assert_eq ! (1 , enc . write (b"a") . unwrap ()) ; } assert_eq ! (& c . get_ref () [..] , NO_PAD_ENGINE . encode ("a") . as_bytes ()) ; assert_eq ! (2 , c . get_ref () . len ()) ; }
};
}
