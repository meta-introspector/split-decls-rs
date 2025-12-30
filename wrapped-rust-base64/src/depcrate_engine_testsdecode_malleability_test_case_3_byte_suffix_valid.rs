// Generated macro for decode_malleability_test_case_3_byte_suffix_valid (function)
macro_rules! Depcrate_engine_testsdecode_malleability_test_case_3_byte_suffix_valid {
() => {
// Module: crate::engine::tests
// Provides: {"decode_malleability_test_case_3_byte_suffix_valid"}
// Dependencies: {}
# [apply (all_engines)] fn decode_malleability_test_case_3_byte_suffix_valid < E : EngineWrapper > (engine_wrapper : E) { assert_eq ! (b"Hello" . as_slice () , & E :: standard () . decode ("SGVsbG8=") . unwrap ()) ; }
};
}
