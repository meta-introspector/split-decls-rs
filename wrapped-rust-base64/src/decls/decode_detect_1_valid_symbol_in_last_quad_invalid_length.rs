macro_rules! deps {
    () => {
        DecodeError!();
        EngineWrapper!();
    };
}

macro_rules! decode_detect_1_valid_symbol_in_last_quad_invalid_length {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_detect_1_valid_symbol_in_last_quad_invalid_length < E : EngineWrapper > (engine_wrapper : E) { for len in (0_usize .. 256) . map (| len | len * 4 + 1) { for mode in all_pad_modes () { let mut input = vec ! [b'A' ; len] ; let engine = E :: standard_with_pad_mode (true , mode) ; assert_eq ! (Err (DecodeError :: InvalidLength (len)) , engine . decode (& input)) ; for _ in 0 .. 3 { input . push (PAD_BYTE) ; assert_eq ! (Err (DecodeError :: InvalidByte (len , PAD_BYTE)) , engine . decode (& input)) ; } } } }
    };
}

decode_detect_1_valid_symbol_in_last_quad_invalid_length!();