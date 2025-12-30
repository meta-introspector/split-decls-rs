// Generated macro for decode_malleability_test_case_2_byte_suffix_no_padding (function)
macro_rules! Depcrate_engine_testsdecode_malleability_test_case_2_byte_suffix_no_padding {
() => {
// Module: crate::engine::tests
// Provides: {"decode_malleability_test_case_2_byte_suffix_no_padding"}
// Dependencies: {}
# [apply (all_engines)] fn decode_malleability_test_case_2_byte_suffix_no_padding < E : EngineWrapper > (engine_wrapper : E) { assert_eq ! (DecodeError :: InvalidPadding , E :: standard () . decode ("SGVsbA") . unwrap_err ()) ; }
};
}
