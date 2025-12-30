// Generated macro for proptest_from_bytes_only_valid_utf8 (function)
macro_rules! Depcrate_testsproptest_from_bytes_only_valid_utf8 {
() => {
// Module: crate::tests
// Provides: {"proptest_from_bytes_only_valid_utf8"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_bytes_only_valid_utf8 (# [strategy (rand_bytes ())] bytes : Vec < u8 >) { let compact_result = CompactString :: from_utf8 (& bytes) ; let word_result = String :: from_utf8 (bytes) ; match (compact_result , word_result) { (Ok (c) , Ok (s)) => prop_assert_eq ! (c , s) , (Err (c_err) , Err (s_err)) => prop_assert_eq ! (c_err , s_err . utf8_error ()) , _ => panic ! ("CompactString and core::str read UTF-8 differently?") , } }
};
}
