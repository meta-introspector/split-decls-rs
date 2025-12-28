macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! encode_nine_bytes_two_writes {
    () => {
        deps!();
        # [test] fn encode_nine_bytes_two_writes () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; let sz = enc . write (b"abcdef") . unwrap () ; assert_eq ! (sz , 6) ; let sz = enc . write (b"ghi") . unwrap () ; assert_eq ! (sz , 3) ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcdefghi") . as_bytes ()) ; }
    };
}

encode_nine_bytes_two_writes!()