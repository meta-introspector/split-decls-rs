// Generated macro for test_resize (function)
macro_rules! Depcrate_teststest_resize {
() => {
// Module: crate::tests
// Provides: {"test_resize"}
// Dependencies: {}
# [test] fn test_resize () { let mut v : SmallVec < i32 , 8 > = SmallVec :: new () ; v . push (1) ; v . resize (5 , 0) ; assert_eq ! (v [..] , [1 , 0 , 0 , 0 , 0] [..]) ; v . resize (2 , - 1) ; assert_eq ! (v [..] , [1 , 0] [..]) ; }
};
}
