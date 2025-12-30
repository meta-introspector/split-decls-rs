// Generated macro for decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol (function)
macro_rules! Depcrate_engine_testsdecode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol {
() => {
// Module: crate::engine::tests
// Provides: {"decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol"}
// Dependencies: {}
# [apply (all_engines)] fn decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol < E : EngineWrapper > (engine_wrapper : E ,) { assert_eq ! (DecodeError :: InvalidLastSymbol (6 , 0x39) , E :: standard () . decode ("SGVsbG9=") . unwrap_err ()) ; }
};
}
