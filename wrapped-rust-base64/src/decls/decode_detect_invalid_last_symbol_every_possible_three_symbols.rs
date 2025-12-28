macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! decode_detect_invalid_last_symbol_every_possible_three_symbols {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_detect_invalid_last_symbol_every_possible_three_symbols < E : EngineWrapper > (engine_wrapper : E ,) { let engine = E :: standard () ; let mut base64_to_bytes = collections :: HashMap :: new () ; let mut bytes = [0_u8 ; 2] ; for b1 in 0_u8 ..= 255 { bytes [0] = b1 ; for b2 in 0_u8 ..= 255 { bytes [1] = b2 ; let mut b64 = vec ! [0_u8 ; 4] ; assert_eq ! (3 , engine . internal_encode (& bytes , & mut b64 [..])) ; let _ = add_padding (3 , & mut b64 [3 ..]) ; let mut v = Vec :: with_capacity (2) ; v . extend_from_slice (& bytes [..]) ; assert ! (base64_to_bytes . insert (b64 , v) . is_none ()) ; } } let mut prefix = Vec :: new () ; let mut input = Vec :: new () ; for _ in 0 .. 256 { input . clear () ; input . extend_from_slice (& prefix) ; let mut symbols = [0_u8 ; 4] ; for & s1 in STANDARD . symbols . iter () { symbols [0] = s1 ; for & s2 in STANDARD . symbols . iter () { symbols [1] = s2 ; for & s3 in STANDARD . symbols . iter () { symbols [2] = s3 ; symbols [3] = PAD_BYTE ; input . truncate (prefix . len ()) ; input . extend_from_slice (& symbols [..]) ; let decoded_prefix_len = prefix . len () / 4 * 3 ; match base64_to_bytes . get (& symbols [..]) { Some (bytes) => { let res = engine . decode (& input) . map (| decoded | decoded [decoded_prefix_len ..] . to_vec ()) ; assert_eq ! (Ok (bytes . clone ()) , res) ; } None => assert_eq ! (Err (DecodeError :: InvalidLastSymbol (2 , s3)) , engine . decode (& symbols [..])) , } } } } prefix . extend_from_slice (b"AAAA") ; } }
    };
}

decode_detect_invalid_last_symbol_every_possible_three_symbols!()