macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! decode_detect_invalid_last_symbol_every_possible_two_symbols {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_detect_invalid_last_symbol_every_possible_two_symbols < E : EngineWrapper > (engine_wrapper : E ,) { let engine = E :: standard () ; let mut base64_to_bytes = collections :: HashMap :: new () ; for b in 0_u8 ..= 255 { let mut b64 = vec ! [0_u8 ; 4] ; assert_eq ! (2 , engine . internal_encode (& [b] , & mut b64 [..])) ; let _ = add_padding (2 , & mut b64 [2 ..]) ; assert ! (base64_to_bytes . insert (b64 , vec ! [b]) . is_none ()) ; } let mut prefix = Vec :: new () ; for _ in 0 .. 256 { let mut clone = prefix . clone () ; let mut symbols = [0_u8 ; 4] ; for & s1 in STANDARD . symbols . iter () { symbols [0] = s1 ; for & s2 in STANDARD . symbols . iter () { symbols [1] = s2 ; symbols [2] = PAD_BYTE ; symbols [3] = PAD_BYTE ; clone . truncate (prefix . len ()) ; clone . extend_from_slice (& symbols [..]) ; let decoded_prefix_len = prefix . len () / 4 * 3 ; match base64_to_bytes . get (& symbols [..]) { Some (bytes) => { let res = engine . decode (& clone) . map (| decoded | decoded [decoded_prefix_len ..] . to_vec ()) ; assert_eq ! (Ok (bytes . clone ()) , res) ; } None => assert_eq ! (Err (DecodeError :: InvalidLastSymbol (1 , s2)) , engine . decode (& symbols [..])) , } } } prefix . extend_from_slice (b"AAAA") ; } }
    };
}

decode_detect_invalid_last_symbol_every_possible_two_symbols!()