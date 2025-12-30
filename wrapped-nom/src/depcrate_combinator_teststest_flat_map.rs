// Generated macro for test_flat_map (function)
macro_rules! Depcrate_combinator_teststest_flat_map {
() => {
// Module: crate::combinator::tests
// Provides: {"test_flat_map"}
// Dependencies: {}
# [test] fn test_flat_map () { let input : & [u8] = & [3 , 100 , 101 , 102 , 103 , 104] [..] ; assert_parse ! (flat_map (u8 , take) . parse (input) , Ok ((& [103 , 104] [..] , & [100 , 101 , 102] [..]))) ; }
};
}
