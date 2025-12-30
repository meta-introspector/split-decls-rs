// Generated macro for test_as_ref (function)
macro_rules! Depcrate_teststest_as_ref {
() => {
// Module: crate::tests
// Provides: {"test_as_ref"}
// Dependencies: {}
# [test] fn test_as_ref () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; assert_eq ! (a . as_ref () , [1]) ; a . push (2) ; assert_eq ! (a . as_ref () , [1 , 2]) ; a . push (3) ; assert_eq ! (a . as_ref () , [1 , 2 , 3]) ; }
};
}
