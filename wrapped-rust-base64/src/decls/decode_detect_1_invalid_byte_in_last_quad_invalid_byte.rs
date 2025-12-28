macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! decode_detect_1_invalid_byte_in_last_quad_invalid_byte {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_detect_1_invalid_byte_in_last_quad_invalid_byte < E : EngineWrapper > (engine_wrapper : E) { for prefix_len in (0_usize .. 256) . map (| len | len * 4) { for mode in all_pad_modes () { let mut input = vec ! [b'A' ; prefix_len] ; input . push (b'*') ; let engine = E :: standard_with_pad_mode (true , mode) ; assert_eq ! (Err (DecodeError :: InvalidByte (prefix_len , b'*')) , engine . decode (& input)) ; for _ in 0 .. 3 { input . push (PAD_BYTE) ; assert_eq ! (Err (DecodeError :: InvalidByte (prefix_len , b'*')) , engine . decode (& input)) ; } } } }
    };
}

decode_detect_1_invalid_byte_in_last_quad_invalid_byte!();