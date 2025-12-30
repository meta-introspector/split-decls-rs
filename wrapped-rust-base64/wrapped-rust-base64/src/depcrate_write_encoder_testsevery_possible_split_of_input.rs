// Generated macro for every_possible_split_of_input (function)
macro_rules! Depcrate_write_encoder_testsevery_possible_split_of_input {
() => {
// Module: crate::write::encoder_tests
// Provides: {"every_possible_split_of_input"}
// Dependencies: {}
# [test] fn every_possible_split_of_input () { let mut rng = rand :: thread_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut stream_encoded = Vec :: < u8 > :: new () ; let mut normal_encoded = String :: new () ; let size = 5_000 ; for i in 0 .. size { orig_data . clear () ; stream_encoded . clear () ; normal_encoded . clear () ; for _ in 0 .. size { orig_data . push (rng . gen ()) ; } let engine = random_engine (& mut rng) ; engine . encode_string (& orig_data , & mut normal_encoded) ; { let mut stream_encoder = EncoderWriter :: new (& mut stream_encoded , & engine) ; stream_encoder . write_all (& orig_data [0 .. i]) . unwrap () ; stream_encoder . write_all (& orig_data [i ..]) . unwrap () ; } assert_eq ! (normal_encoded , str :: from_utf8 (& stream_encoded) . unwrap ()) ; } }
};
}
