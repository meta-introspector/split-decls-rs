// Generated macro for test_update_reader (function)
macro_rules! Depcrate_testtest_update_reader {
() => {
// Module: crate::test
// Provides: {"test_update_reader"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn test_update_reader () -> Result < () , std :: io :: Error > { let mut input = vec ! [0 ; 1_000_000] ; paint_test_input (& mut input) ; assert_eq ! (crate :: Hasher :: new () . update_reader (& input [..]) ?. finalize () , crate :: hash (& input) ,) ; Ok (()) }
};
}
