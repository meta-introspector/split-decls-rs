// Generated macro for proptest_from_utf8_lossy (function)
macro_rules! Depcrate_testsproptest_from_utf8_lossy {
() => {
// Module: crate::tests
// Provides: {"proptest_from_utf8_lossy"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_utf8_lossy (# [strategy (rand_bytes ())] bytes : Vec < u8 >) { let compact = CompactString :: from_utf8_lossy (& bytes) ; let control = String :: from_utf8_lossy (& bytes) ; assert_eq ! (compact , control) ; assert_eq ! (compact . len () , control . len ()) ; }
};
}
