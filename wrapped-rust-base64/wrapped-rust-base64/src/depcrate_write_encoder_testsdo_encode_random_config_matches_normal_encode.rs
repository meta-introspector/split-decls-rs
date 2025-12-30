// Generated macro for do_encode_random_config_matches_normal_encode (function)
macro_rules! Depcrate_write_encoder_testsdo_encode_random_config_matches_normal_encode {
() => {
// Module: crate::write::encoder_tests
// Provides: {"do_encode_random_config_matches_normal_encode"}
// Dependencies: {}
fn do_encode_random_config_matches_normal_encode (max_input_len : usize) { let mut rng = rand :: thread_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut stream_encoded = Vec :: < u8 > :: new () ; let mut normal_encoded = String :: new () ; for _ in 0 .. 1_000 { orig_data . clear () ; stream_encoded . clear () ; normal_encoded . clear () ; let orig_len : usize = rng . gen_range (100 .. 20_000) ; for _ in 0 .. orig_len { orig_data . push (rng . gen ()) ; } let engine = random_engine (& mut rng) ; engine . encode_string (& orig_data , & mut normal_encoded) ; { let mut stream_encoder = EncoderWriter :: new (& mut stream_encoded , & engine) ; let mut bytes_consumed = 0 ; while bytes_consumed < orig_len { let input_len : usize = cmp :: min (rng . gen_range (0 .. max_input_len) , orig_len - bytes_consumed) ; stream_encoder . write_all (& orig_data [bytes_consumed .. bytes_consumed + input_len]) . unwrap () ; bytes_consumed += input_len ; } let _ = stream_encoder . finish () . unwrap () ; assert_eq ! (orig_len , bytes_consumed) ; } assert_eq ! (normal_encoded , str :: from_utf8 (& stream_encoded) . unwrap ()) ; } }
};
}
