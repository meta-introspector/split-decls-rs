macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! finish_writes_extra_byte {
    () => {
        deps!();
        # [test] fn finish_writes_extra_byte () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & URL_SAFE_ENGINE) ; assert_eq ! (6 , enc . write (b"abcdef") . unwrap ()) ; assert_eq ! (1 , enc . write (b"g") . unwrap ()) ; let _ = enc . finish () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , URL_SAFE_ENGINE . encode ("abcdefg") . as_bytes ()) ; }
    };
}

finish_writes_extra_byte!()