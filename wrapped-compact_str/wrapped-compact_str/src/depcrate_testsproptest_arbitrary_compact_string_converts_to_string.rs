// Generated macro for proptest_arbitrary_compact_string_converts_to_string (function)
macro_rules! Depcrate_testsproptest_arbitrary_compact_string_converts_to_string {
() => {
// Module: crate::tests
// Provides: {"proptest_arbitrary_compact_string_converts_to_string"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_arbitrary_compact_string_converts_to_string (# [strategy (rand_unicode ())] word : String) { let compact = CompactString :: new (& word) ; let result = String :: from (compact) ; prop_assert_eq ! (result . len () , word . len ()) ; prop_assert_eq ! (result , word) ; }
};
}
