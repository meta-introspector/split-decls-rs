macro_rules! deps {
    () => {
        Engine!();
        DecodeError!();
        DecodePaddingMode!();
    };
}

macro_rules! do_invalid_trailing_byte {
    () => {
        deps!();
        fn do_invalid_trailing_byte (engine : impl Engine , mode : DecodePaddingMode) { for last_byte in [b'*' , b'\n'] { for num_prefix_quads in 0 .. 256 { let mut s : String = "ABCD" . repeat (num_prefix_quads) ; s . push_str ("Cg==") ; let mut input = s . into_bytes () ; input . push (last_byte) ; assert_eq ! (Err (DecodeError :: InvalidByte (num_prefix_quads * 4 + 4 , last_byte)) , engine . decode (& input) , "mode: {:?}, input: {}" , mode , String :: from_utf8 (input) . unwrap ()) ; } } }
    };
}

do_invalid_trailing_byte!()