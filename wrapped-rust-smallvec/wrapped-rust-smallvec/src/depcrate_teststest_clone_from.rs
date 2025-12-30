// Generated macro for test_clone_from (function)
macro_rules! Depcrate_teststest_clone_from {
() => {
// Module: crate::tests
// Provides: {"test_clone_from"}
// Dependencies: {}
# [test] fn test_clone_from () { let mut a : SmallVec < u8 , 2 > = SmallVec :: new () ; a . push (1) ; a . push (2) ; a . push (3) ; let mut b : SmallVec < u8 , 2 > = SmallVec :: new () ; b . push (10) ; let mut c : SmallVec < u8 , 2 > = SmallVec :: new () ; c . push (20) ; c . push (21) ; c . push (22) ; a . clone_from (& b) ; assert_eq ! (&* a , & [10]) ; b . clone_from (& c) ; assert_eq ! (&* b , & [20 , 21 , 22]) ; }
};
}
