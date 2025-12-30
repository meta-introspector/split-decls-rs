// Generated macro for simple (function)
macro_rules! Depcrate_read_decoder_testssimple {
() => {
// Module: crate::read::decoder_tests
// Provides: {"simple"}
// Dependencies: {}
# [test] fn simple () { let tests : & [(& [u8] , & [u8])] = & [(& b"0" [..] , & b"MA==" [..]) , (b"01" , b"MDE=") , (b"012" , b"MDEy") , (b"0123" , b"MDEyMw==") , (b"01234" , b"MDEyMzQ=") , (b"012345" , b"MDEyMzQ1") , (b"0123456" , b"MDEyMzQ1Ng==") , (b"01234567" , b"MDEyMzQ1Njc=") , (b"012345678" , b"MDEyMzQ1Njc4") , (b"0123456789" , b"MDEyMzQ1Njc4OQ==") ,] [..] ; for (text_expected , base64data) in tests . iter () { for n in 1 .. base64data . len () + 1 { let mut wrapped_reader = io :: Cursor :: new (base64data) ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & STANDARD) ; let mut text_got = Vec :: new () ; let mut buffer = vec ! [0u8 ; n] ; while let Ok (read) = decoder . read (& mut buffer [..]) { if read == 0 { break ; } text_got . extend_from_slice (& buffer [.. read]) ; } assert_eq ! (text_got , * text_expected , "\nGot: {}\nExpected: {}" , String :: from_utf8_lossy (& text_got [..]) , String :: from_utf8_lossy (text_expected)) ; } } }
};
}
