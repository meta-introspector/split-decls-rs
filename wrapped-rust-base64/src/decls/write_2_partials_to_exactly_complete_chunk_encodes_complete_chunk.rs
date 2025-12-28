macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! write_2_partials_to_exactly_complete_chunk_encodes_complete_chunk {
    () => {
        deps!();
        # [test] fn write_2_partials_to_exactly_complete_chunk_encodes_complete_chunk () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & NO_PAD_ENGINE) ; assert_eq ! (1 , enc . write (b"a") . unwrap ()) ; assert_eq ! (2 , enc . write (b"bc") . unwrap ()) ; let _ = enc . finish () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , NO_PAD_ENGINE . encode ("abc") . as_bytes ()) ; assert_eq ! (4 , c . get_ref () . len ()) ; }
    };
}

write_2_partials_to_exactly_complete_chunk_encodes_complete_chunk!()