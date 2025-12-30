// Generated macro for test_all_consuming (function)
macro_rules! Depcrate_combinator_teststest_all_consuming {
() => {
// Module: crate::combinator::tests
// Provides: {"test_all_consuming"}
// Dependencies: {}
# [test] fn test_all_consuming () { let input : & [u8] = & [100 , 101 , 102] [..] ; assert_parse ! (all_consuming (take (2usize)) . parse (input) , Err (Err :: Error ((& [102] [..] , ErrorKind :: Eof)))) ; assert_parse ! (all_consuming (take (3usize)) . parse (input) , Ok ((& [] [..] , & [100 , 101 , 102] [..]))) ; }
};
}
