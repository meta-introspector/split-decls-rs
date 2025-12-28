macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! encode_three_bytes {
    () => {
        deps!();
        # [test] fn encode_three_bytes () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; let sz = enc . write (b"abc") . unwrap () ; assert_eq ! (sz , 3) ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abc") . as_bytes ()) ; }
    };
}

encode_three_bytes!()