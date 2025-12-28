macro_rules! deps {
    () => {
        DecoderReader!();
    };
}

macro_rules! trailing_junk {
    () => {
        deps!();
        # [test] fn trailing_junk () { let tests : & [& [u8]] = & [& b"MDEyMzQ1Njc4*!@#$%^&" [..] , b"MDEyMzQ1Njc4OQ== "] [..] ; for base64data in tests . iter () { for n in 1 .. base64data . len () + 1 { let mut wrapped_reader = io :: Cursor :: new (base64data) ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & STANDARD) ; let mut buffer = vec ! [0u8 ; n] ; let mut saw_error = false ; loop { match decoder . read (& mut buffer [..]) { Err (_) => { saw_error = true ; break ; } Ok (0) => break , Ok (_len) => () , } } assert ! (saw_error) ; } } }
    };
}

trailing_junk!();