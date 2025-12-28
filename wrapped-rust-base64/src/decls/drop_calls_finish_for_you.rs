macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! drop_calls_finish_for_you {
    () => {
        deps!();
        # [test] fn drop_calls_finish_for_you () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & NO_PAD_ENGINE) ; assert_eq ! (1 , enc . write (b"a") . unwrap ()) ; } assert_eq ! (& c . get_ref () [..] , NO_PAD_ENGINE . encode ("a") . as_bytes ()) ; assert_eq ! (2 , c . get_ref () . len ()) ; }
    };
}

drop_calls_finish_for_you!();