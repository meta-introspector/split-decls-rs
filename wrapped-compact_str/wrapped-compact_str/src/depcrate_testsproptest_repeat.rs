// Generated macro for proptest_repeat (function)
macro_rules! Depcrate_testsproptest_repeat {
() => {
// Module: crate::tests
// Provides: {"proptest_repeat"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_repeat (n : u16 , s : String) { let compact = CompactString :: new (& s) . repeat (n as usize) ; let control = s . repeat (n as usize) ; assert_eq ! (compact , control) ; }
};
}
