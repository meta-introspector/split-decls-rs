macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! encode_with_padding_multiple_writes {
    () => {
        deps!();
        # [test] fn encode_with_padding_multiple_writes () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; assert_eq ! (1 , enc . write (b"a") . unwrap ()) ; assert_eq ! (2 , enc . write (b"bc") . unwrap ()) ; assert_eq ! (3 , enc . write (b"def") . unwrap ()) ; assert_eq ! (1 , enc . write (b"g") . unwrap ()) ; enc . flush () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcdefg") . as_bytes ()) ; }
    };
}

encode_with_padding_multiple_writes!();