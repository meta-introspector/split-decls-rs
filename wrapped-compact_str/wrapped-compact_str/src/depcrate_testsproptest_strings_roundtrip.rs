// Generated macro for proptest_strings_roundtrip (function)
macro_rules! Depcrate_testsproptest_strings_roundtrip {
() => {
// Module: crate::tests
// Provides: {"proptest_strings_roundtrip"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_strings_roundtrip (# [strategy (rand_unicode ())] word : String) { let compact = CompactString :: new (& word) ; prop_assert_eq ! (& word , & compact) ; }
};
}
