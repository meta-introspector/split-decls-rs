// Generated macro for test_eq (function)
macro_rules! Depcrate_teststest_eq {
() => {
// Module: crate::tests
// Provides: {"test_eq"}
// Dependencies: {}
# [test] fn test_eq () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; let mut b : SmallVec < u32 , 2 > = SmallVec :: new () ; let mut c : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; a . push (2) ; b . push (1) ; b . push (2) ; c . push (3) ; c . push (4) ; assert ! (a == b) ; assert ! (a != c) ; }
};
}
