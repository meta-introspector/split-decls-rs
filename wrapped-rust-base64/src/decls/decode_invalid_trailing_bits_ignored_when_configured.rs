macro_rules! deps {
    () => {
        EngineWrapper!();
        Engine!();
    };
}

macro_rules! decode_invalid_trailing_bits_ignored_when_configured {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_invalid_trailing_bits_ignored_when_configured < E : EngineWrapper > (engine_wrapper : E) { let strict = E :: standard () ; let forgiving = E :: standard_allow_trailing_bits () ; fn assert_tolerant_decode < E : Engine > (engine : & E , input : & mut String , b64_prefix_len : usize , expected_decode_bytes : Vec < u8 > , data : & str ,) { let prefixed = prefixed_data (input , b64_prefix_len , data) ; let decoded = engine . decode (prefixed) ; let decoded_prefix_len = b64_prefix_len / 4 * 3 ; assert_eq ! (Ok (expected_decode_bytes) , decoded . map (| v | v [decoded_prefix_len ..] . to_vec ())) ; } let mut prefix = String :: new () ; for _ in 0 .. 256 { let mut input = prefix . clone () ; assert ! (strict . decode (prefixed_data (& mut input , prefix . len () , "/w==")) . is_ok ()) ; assert ! (strict . decode (prefixed_data (& mut input , prefix . len () , "iYU=")) . is_ok ()) ; assert_tolerant_decode (& forgiving , & mut input , prefix . len () , vec ! [255] , "/x==") ; assert_tolerant_decode (& forgiving , & mut input , prefix . len () , vec ! [137 , 133] , "iYV=") ; assert_tolerant_decode (& forgiving , & mut input , prefix . len () , vec ! [255] , "/y==") ; assert_tolerant_decode (& forgiving , & mut input , prefix . len () , vec ! [137 , 133] , "iYW=") ; assert_tolerant_decode (& forgiving , & mut input , prefix . len () , vec ! [255] , "/z==") ; assert_tolerant_decode (& forgiving , & mut input , prefix . len () , vec ! [137 , 133] , "iYX=") ; prefix . push_str ("AAAA") ; } }
    };
}

decode_invalid_trailing_bits_ignored_when_configured!();