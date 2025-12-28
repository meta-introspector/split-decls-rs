macro_rules! deps {
    () => {
        DecodeError!();
        EngineWrapper!();
    };
}

macro_rules! decode_detect_invalid_last_symbol {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_detect_invalid_last_symbol < E : EngineWrapper > (engine_wrapper : E) { let engine = E :: standard () ; assert_eq ! (Ok (vec ! [0x89 , 0x85]) , engine . decode ("iYU=")) ; assert_eq ! (Ok (vec ! [0xFF]) , engine . decode ("/w==")) ; for (suffix , offset) in vec ! [("/x==" , 1_usize) , ("/z==" , 1_usize) , ("/0==" , 1_usize) , ("/9==" , 1_usize) , ("/+==" , 1_usize) , ("//==" , 1_usize) , ("iYV=" , 2_usize) , ("iYW=" , 2_usize) , ("iYX=" , 2_usize) ,] { for prefix_quads in 0 .. 256 { let mut encoded = "AAAA" . repeat (prefix_quads) ; encoded . push_str (suffix) ; assert_eq ! (Err (DecodeError :: InvalidLastSymbol (encoded . len () - 4 + offset , suffix . as_bytes () [offset] ,)) , engine . decode (encoded . as_str ())) ; } } }
    };
}

decode_detect_invalid_last_symbol!()