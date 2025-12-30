// Generated macro for test_map_parser (function)
macro_rules! Depcrate_combinator_teststest_map_parser {
() => {
// Module: crate::combinator::tests
// Provides: {"test_map_parser"}
// Dependencies: {}
# [test] fn test_map_parser () { let input : & [u8] = & [100 , 101 , 102 , 103 , 104] [..] ; assert_parse ! (map_parser (take (4usize) , take (2usize)) . parse (input) , Ok ((& [104] [..] , & [100 , 101] [..]))) ; }
};
}
