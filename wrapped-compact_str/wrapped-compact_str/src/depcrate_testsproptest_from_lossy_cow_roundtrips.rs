// Generated macro for proptest_from_lossy_cow_roundtrips (function)
macro_rules! Depcrate_testsproptest_from_lossy_cow_roundtrips {
() => {
// Module: crate::tests
// Provides: {"proptest_from_lossy_cow_roundtrips"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_lossy_cow_roundtrips (# [strategy (rand_bytes ())] bytes : Vec < u8 >) { let cow = String :: from_utf8_lossy (& bytes [..]) ; let compact = CompactString :: from (cow . clone ()) ; prop_assert_eq ! (cow , compact) ; }
};
}
