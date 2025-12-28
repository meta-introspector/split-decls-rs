macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! rfc_test_vectors_std_alphabet {
    () => {
        deps!();
        # [apply (all_engines)] fn rfc_test_vectors_std_alphabet < E : EngineWrapper > (engine_wrapper : E) { let data = vec ! [("" , "") , ("f" , "Zg==") , ("fo" , "Zm8=") , ("foo" , "Zm9v") , ("foob" , "Zm9vYg==") , ("fooba" , "Zm9vYmE=") , ("foobar" , "Zm9vYmFy") ,] ; let engine = E :: standard () ; let engine_no_padding = E :: standard_unpadded () ; for (orig , encoded) in & data { let encoded_without_padding = encoded . trim_end_matches ('=') ; { let mut encode_buf = [0_u8 ; 8] ; let mut decode_buf = [0_u8 ; 6] ; let encode_len = engine_no_padding . internal_encode (orig . as_bytes () , & mut encode_buf [..]) ; assert_eq ! (& encoded_without_padding , & std :: str :: from_utf8 (& encode_buf [0 .. encode_len]) . unwrap ()) ; let decode_len = engine_no_padding . decode_slice_unchecked (encoded_without_padding . as_bytes () , & mut decode_buf [..]) . unwrap () ; assert_eq ! (orig . len () , decode_len) ; assert_eq ! (orig , & std :: str :: from_utf8 (& decode_buf [0 .. decode_len]) . unwrap ()) ; if encoded . as_bytes () . contains (& PAD_BYTE) { assert_eq ! (Err (DecodeError :: InvalidPadding) , engine_no_padding . decode (encoded)) } } { let mut encode_buf = [0_u8 ; 8] ; let mut decode_buf = [0_u8 ; 6] ; let encode_len = engine . internal_encode (orig . as_bytes () , & mut encode_buf [..]) ; assert_eq ! (& encoded_without_padding , & std :: str :: from_utf8 (& encode_buf [0 .. encode_len]) . unwrap ()) ; let pad_len = add_padding (encode_len , & mut encode_buf [encode_len ..]) ; assert_eq ! (encoded . as_bytes () , & encode_buf [.. encode_len + pad_len]) ; let decode_len = engine . decode_slice_unchecked (encoded . as_bytes () , & mut decode_buf [..]) . unwrap () ; assert_eq ! (orig . len () , decode_len) ; assert_eq ! (orig , & std :: str :: from_utf8 (& decode_buf [0 .. decode_len]) . unwrap ()) ; if encoded . as_bytes () . contains (& PAD_BYTE) { assert_eq ! (Err (DecodeError :: InvalidPadding) , engine . decode (encoded_without_padding)) } } } }
    };
}

rfc_test_vectors_std_alphabet!();