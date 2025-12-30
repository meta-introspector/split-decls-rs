// Generated macro for proptest_strings_allocated_properly (function)
macro_rules! Depcrate_testsproptest_strings_allocated_properly {
() => {
// Module: crate::tests
// Provides: {"proptest_strings_allocated_properly"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_strings_allocated_properly (# [strategy (rand_unicode ())] word : String) { let compact = CompactString :: new (word) ; assert_allocated_properly (& compact) ; }
};
}
