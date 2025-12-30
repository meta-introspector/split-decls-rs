// Generated macro for decode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes (function)
macro_rules! Depcrate_engine_testsdecode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes {
() => {
// Module: crate::engine::tests
// Provides: {"decode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes"}
// Dependencies: {}
# [apply (all_engines_except_decoder_reader)] fn decode_invalid_trailing_padding_as_invalid_byte_at_first_byte_all_modes < E : EngineWrapper > (engine_wrapper : E ,) { for mode in all_pad_modes () { do_invalid_trailing_padding_as_invalid_byte_at_first_padding (E :: standard_with_pad_mode (true , mode) , mode ,) ; } }
};
}
