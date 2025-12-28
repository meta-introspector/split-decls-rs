macro_rules! deps {
    () => {
        EncoderWriter!();
    };
}

macro_rules! write_1_chunk_encodes_complete_chunk {
    () => {
        deps!();
        # [test] fn write_1_chunk_encodes_complete_chunk () { let mut c = Cursor :: new (Vec :: new ()) ; { let mut enc = EncoderWriter :: new (& mut c , & NO_PAD_ENGINE) ; assert_eq ! (3 , enc . write (b"abc") . unwrap ()) ; let _ = enc . finish () . unwrap () ; } assert_eq ! (& c . get_ref () [..] , NO_PAD_ENGINE . encode ("abc") . as_bytes ()) ; assert_eq ! (4 , c . get_ref () . len ()) ; }
    };
}

write_1_chunk_encodes_complete_chunk!();