// Generated macro for test_as_mut (function)
macro_rules! Depcrate_teststest_as_mut {
() => {
// Module: crate::tests
// Provides: {"test_as_mut"}
// Dependencies: {}
# [test] fn test_as_mut () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; assert_eq ! (a . as_mut () , [1]) ; a . push (2) ; assert_eq ! (a . as_mut () , [1 , 2]) ; a . push (3) ; assert_eq ! (a . as_mut () , [1 , 2 , 3]) ; a . as_mut () [1] = 4 ; assert_eq ! (a . as_mut () , [1 , 4 , 3]) ; }
};
}
