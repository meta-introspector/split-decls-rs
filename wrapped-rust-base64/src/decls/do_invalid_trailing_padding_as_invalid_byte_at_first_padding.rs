macro_rules! deps {
    () => {
        Engine!();
        DecodeError!();
        DecodePaddingMode!();
    };
}

macro_rules! do_invalid_trailing_padding_as_invalid_byte_at_first_padding {
    () => {
        deps!();
        fn do_invalid_trailing_padding_as_invalid_byte_at_first_padding (engine : impl Engine , mode : DecodePaddingMode ,) { for num_prefix_quads in 0 .. 256 { for (suffix , pad_offset) in [("AA===" , 2) , ("AAA==" , 3) , ("AAAA=" , 4)] { let mut s : String = "ABCD" . repeat (num_prefix_quads) ; s . push_str (suffix) ; assert_eq ! (Err (DecodeError :: InvalidByte (num_prefix_quads * 4 + pad_offset , PAD_BYTE)) , engine . decode (& s) , "mode: {:?}, input: {}" , mode , s) ; } } }
    };
}

do_invalid_trailing_padding_as_invalid_byte_at_first_padding!();