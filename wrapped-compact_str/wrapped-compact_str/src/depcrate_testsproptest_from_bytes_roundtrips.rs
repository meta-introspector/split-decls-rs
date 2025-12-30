// Generated macro for proptest_from_bytes_roundtrips (function)
macro_rules! Depcrate_testsproptest_from_bytes_roundtrips {
() => {
// Module: crate::tests
// Provides: {"proptest_from_bytes_roundtrips"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_bytes_roundtrips (# [strategy (rand_unicode ())] word : String) { let bytes = word . into_bytes () ; let compact = CompactString :: from_utf8 (& bytes) . unwrap () ; let word = String :: from_utf8 (bytes) . unwrap () ; prop_assert_eq ! (compact , word) ; }
};
}
