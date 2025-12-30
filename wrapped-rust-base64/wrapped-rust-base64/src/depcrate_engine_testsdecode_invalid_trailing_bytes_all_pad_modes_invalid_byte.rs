// Generated macro for decode_invalid_trailing_bytes_all_pad_modes_invalid_byte (function)
macro_rules! Depcrate_engine_testsdecode_invalid_trailing_bytes_all_pad_modes_invalid_byte {
() => {
// Module: crate::engine::tests
// Provides: {"decode_invalid_trailing_bytes_all_pad_modes_invalid_byte"}
// Dependencies: {}
# [doc = " 1 trailing byte that's not padding is detected as invalid byte even though there's padding"] # [doc = " in the middle of the input. This is essentially mandating the eager check for 1 trailing byte"] # [doc = " to catch the \\n suffix case."] # [apply (all_engines_except_decoder_reader)] fn decode_invalid_trailing_bytes_all_pad_modes_invalid_byte < E : EngineWrapper > (engine_wrapper : E) { for mode in all_pad_modes () { do_invalid_trailing_byte (E :: standard_with_pad_mode (true , mode) , mode) ; } }
};
}
