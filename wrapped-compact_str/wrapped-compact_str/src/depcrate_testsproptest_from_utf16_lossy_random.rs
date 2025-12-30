// Generated macro for proptest_from_utf16_lossy_random (function)
macro_rules! Depcrate_testsproptest_from_utf16_lossy_random {
() => {
// Module: crate::tests
// Provides: {"proptest_from_utf16_lossy_random"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_utf16_lossy_random (# [strategy (rand_u16s ())] buf : Vec < u16 >) { let control = String :: from_utf16_lossy (& buf) ; let compact = CompactString :: from_utf16_lossy (& buf) ; assert_eq ! (compact , control) ; }
};
}
