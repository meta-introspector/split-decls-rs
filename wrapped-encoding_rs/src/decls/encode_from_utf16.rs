macro_rules! deps {
    () => {
        CoderResult!();
        Encoding!();
    };
}

macro_rules! encode_from_utf16 {
    () => {
        deps!();
        pub fn encode_from_utf16 (encoding : & 'static Encoding , string : & [u16] , expect : & [u8]) { let mut encoder = encoding . new_encoder () ; let mut dest : Vec < u8 > = Vec :: with_capacity (10 * (string . len () + 1)) ; let capacity = dest . capacity () ; dest . resize (capacity , 0u8) ; let (complete , read , written , _) = encoder . encode_from_utf16 (string , & mut dest , true) ; match complete { CoderResult :: InputEmpty => { } CoderResult :: OutputFull => { unreachable ! () ; } } assert_eq ! (read , string . len ()) ; dest . truncate (written) ; assert_eq ! (& dest [..] , expect) ; }
    };
}

encode_from_utf16!()