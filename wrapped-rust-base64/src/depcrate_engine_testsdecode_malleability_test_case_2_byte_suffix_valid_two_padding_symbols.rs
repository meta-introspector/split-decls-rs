// Generated macro for decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols (function)
macro_rules! Depcrate_engine_testsdecode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols {
() => {
// Module: crate::engine::tests
// Provides: {"decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols"}
// Dependencies: {}
# [apply (all_engines)] fn decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols < E : EngineWrapper > (engine_wrapper : E ,) { assert_eq ! (b"Hell" . as_slice () , & E :: standard () . decode ("SGVsbA==") . unwrap ()) ; }
};
}
