macro_rules! deps {
    () => {
        Engine!();
        DecodeError!();
    };
}

macro_rules! decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad {
    () => {
        deps!();
        fn decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad (engine : impl Engine , suffixes : & [(& str , usize)] ,) { let mut rng = seeded_rng () ; let prefix_quads_range = distributions :: Uniform :: from (0 ..= 256) ; for _ in 0 .. 100_000 { for (suffix , suffix_offset) in suffixes . iter () { let mut s = "AAAA" . repeat (prefix_quads_range . sample (& mut rng)) ; s . push_str (suffix) ; let mut encoded = s . into_bytes () ; let last_non_padding_offset = encoded . len () - 1 - suffix_offset ; let padding_end = rng . gen_range (0 .. last_non_padding_offset) ; let padding_len = rng . gen_range (1 ..= usize :: min (100 , padding_end + 1)) ; let padding_start = padding_end . saturating_sub (padding_len) ; encoded [padding_start ..= padding_end] . fill (PAD_BYTE) ; assert_ne ! (PAD_BYTE , encoded [last_non_padding_offset]) ; assert_eq ! (Err (DecodeError :: InvalidByte (padding_start , PAD_BYTE)) , engine . decode (& encoded) , "len: {}, input: {}" , encoded . len () , String :: from_utf8 (encoded) . unwrap ()) ; } } }
    };
}

decode_padding_before_final_non_padding_char_error_invalid_byte_at_first_pad!();