// Generated macro for proptest_from_utf16_lossy_roundtrips (function)
macro_rules! Depcrate_testsproptest_from_utf16_lossy_roundtrips {
() => {
// Module: crate::tests
// Provides: {"proptest_from_utf16_lossy_roundtrips"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_utf16_lossy_roundtrips (# [strategy (rand_unicode ())] control : String) { let utf16_buf : Vec < u16 > = control . encode_utf16 () . collect () ; let compact = CompactString :: from_utf16_lossy (utf16_buf) ; assert_eq ! (compact , control) ; }
};
}
