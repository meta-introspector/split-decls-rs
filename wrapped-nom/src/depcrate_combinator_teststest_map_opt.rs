// Generated macro for test_map_opt (function)
macro_rules! Depcrate_combinator_teststest_map_opt {
() => {
// Module: crate::combinator::tests
// Provides: {"test_map_opt"}
// Dependencies: {}
# [test] fn test_map_opt () { let input : & [u8] = & [50] [..] ; assert_parse ! (map_opt (u8 , | u | if u < 20 { Some (u) } else { None }) . parse (input) , Err (Err :: Error ((& [50] [..] , ErrorKind :: MapOpt)))) ; assert_parse ! (map_opt (u8 , | u | if u > 20 { Some (u) } else { None }) . parse (input) , Ok ((& [] [..] , 50))) ; }
};
}
