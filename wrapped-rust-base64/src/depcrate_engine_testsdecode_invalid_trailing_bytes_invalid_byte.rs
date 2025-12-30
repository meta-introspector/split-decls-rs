// Generated macro for decode_invalid_trailing_bytes_invalid_byte (function)
macro_rules! Depcrate_engine_testsdecode_invalid_trailing_bytes_invalid_byte {
() => {
// Module: crate::engine::tests
// Provides: {"decode_invalid_trailing_bytes_invalid_byte"}
// Dependencies: {}
# [apply (all_engines)] fn decode_invalid_trailing_bytes_invalid_byte < E : EngineWrapper > (engine_wrapper : E) { for mode in pad_modes_allowing_padding () { do_invalid_trailing_byte (E :: standard_with_pad_mode (true , mode) , mode) ; } }
};
}
