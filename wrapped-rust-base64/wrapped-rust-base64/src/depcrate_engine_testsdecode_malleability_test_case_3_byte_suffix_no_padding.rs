// Generated macro for decode_malleability_test_case_3_byte_suffix_no_padding (function)
macro_rules! Depcrate_engine_testsdecode_malleability_test_case_3_byte_suffix_no_padding {
() => {
// Module: crate::engine::tests
// Provides: {"decode_malleability_test_case_3_byte_suffix_no_padding"}
// Dependencies: {}
# [apply (all_engines)] fn decode_malleability_test_case_3_byte_suffix_no_padding < E : EngineWrapper > (engine_wrapper : E) { assert_eq ! (DecodeError :: InvalidPadding , E :: standard () . decode ("SGVsbG9") . unwrap_err ()) ; }
};
}
