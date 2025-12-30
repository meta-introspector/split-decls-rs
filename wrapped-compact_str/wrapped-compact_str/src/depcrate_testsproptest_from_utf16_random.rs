// Generated macro for proptest_from_utf16_random (function)
macro_rules! Depcrate_testsproptest_from_utf16_random {
() => {
// Module: crate::tests
// Provides: {"proptest_from_utf16_random"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_utf16_random (# [strategy (rand_u16s ())] buf : Vec < u16 >) { let compact = CompactString :: from_utf16 (& buf) ; let std_str = String :: from_utf16 (& buf) ; match (compact , std_str) { (Ok (c) , Ok (s)) => assert_eq ! (c , s) , (Err (_) , Err (_)) => () , (c_res , s_res) => panic ! ("CompactString and String decode UTF-16 differently? {:?} {:?}" , c_res , s_res) , } }
};
}
