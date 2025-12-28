macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! encode_one_then_five_bytes {
    () => {
        deps!();
        # [test] fn encode_one_then_five_bytes () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; let sz = enc . write (b"a") . unwrap () ; assert_eq ! (sz , 1) ; let sz = enc . write (b"bcdef") . unwrap () ; assert_eq ! (sz , 5) ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcdef") . as_bytes ()) ; }
    };
}

encode_one_then_five_bytes!();