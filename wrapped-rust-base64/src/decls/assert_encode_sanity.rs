macro_rules! assert_encode_sanity {
    () => {
        pub fn assert_encode_sanity (encoded : & str , padded : bool , input_len : usize) { let input_rem = input_len % 3 ; let expected_padding_len = if input_rem > 0 { if padded { 3 - input_rem } else { 0 } } else { 0 } ; let expected_encoded_len = encoded_len (input_len , padded) . unwrap () ; assert_eq ! (expected_encoded_len , encoded . len ()) ; let padding_len = encoded . chars () . filter (| & c | c == '=') . count () ; assert_eq ! (expected_padding_len , padding_len) ; let _ = str :: from_utf8 (encoded . as_bytes ()) . expect ("Base64 should be valid utf8") ; }
    };
}

assert_encode_sanity!();