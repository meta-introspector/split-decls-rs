// Generated macro for retrying_writes_that_error_with_interrupted_works (function)
macro_rules! Depcrate_write_encoder_testsretrying_writes_that_error_with_interrupted_works {
() => {
// Module: crate::write::encoder_tests
// Provides: {"retrying_writes_that_error_with_interrupted_works"}
// Dependencies: {}
# [test] fn retrying_writes_that_error_with_interrupted_works () { let mut rng = rand :: thread_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut stream_encoded = Vec :: < u8 > :: new () ; let mut normal_encoded = String :: new () ; for _ in 0 .. 1_000 { orig_data . clear () ; stream_encoded . clear () ; normal_encoded . clear () ; let orig_len : usize = rng . gen_range (100 .. 20_000) ; for _ in 0 .. orig_len { orig_data . push (rng . gen ()) ; } let engine = random_engine (& mut rng) ; engine . encode_string (& orig_data , & mut normal_encoded) ; { let mut interrupt_rng = rand :: thread_rng () ; let mut interrupting_writer = InterruptingWriter { w : & mut stream_encoded , rng : & mut interrupt_rng , fraction : 0.8 , } ; let mut stream_encoder = EncoderWriter :: new (& mut interrupting_writer , & engine) ; let mut bytes_consumed = 0 ; while bytes_consumed < orig_len { let input_len : usize = cmp :: min (rng . gen_range (0 .. 10) , orig_len - bytes_consumed) ; retry_interrupted_write_all (& mut stream_encoder , & orig_data [bytes_consumed .. bytes_consumed + input_len] ,) . unwrap () ; bytes_consumed += input_len ; } loop { let res = stream_encoder . finish () ; match res { Ok (_) => break , Err (e) => match e . kind () { io :: ErrorKind :: Interrupted => continue , _ => panic ! ("{:?}" , e) , } , } } assert_eq ! (orig_len , bytes_consumed) ; } assert_eq ! (normal_encoded , str :: from_utf8 (& stream_encoded) . unwrap ()) ; } }
};
}
