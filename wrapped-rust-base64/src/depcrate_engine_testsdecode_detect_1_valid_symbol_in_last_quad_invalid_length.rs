// Generated macro for decode_detect_1_valid_symbol_in_last_quad_invalid_length (function)
macro_rules! Depcrate_engine_testsdecode_detect_1_valid_symbol_in_last_quad_invalid_length {
() => {
// Module: crate::engine::tests
// Provides: {"decode_detect_1_valid_symbol_in_last_quad_invalid_length"}
// Dependencies: {}
# [apply (all_engines)] fn decode_detect_1_valid_symbol_in_last_quad_invalid_length < E : EngineWrapper > (engine_wrapper : E) { for len in (0_usize .. 256) . map (| len | len * 4 + 1) { for mode in all_pad_modes () { let mut input = vec ! [b'A' ; len] ; let engine = E :: standard_with_pad_mode (true , mode) ; assert_eq ! (Err (DecodeError :: InvalidLength (len)) , engine . decode (& input)) ; for _ in 0 .. 3 { input . push (PAD_BYTE) ; assert_eq ! (Err (DecodeError :: InvalidByte (len , PAD_BYTE)) , engine . decode (& input)) ; } } } }
};
}
