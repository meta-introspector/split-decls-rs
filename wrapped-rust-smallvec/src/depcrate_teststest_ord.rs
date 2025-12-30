// Generated macro for test_ord (function)
macro_rules! Depcrate_teststest_ord {
() => {
// Module: crate::tests
// Provides: {"test_ord"}
// Dependencies: {}
# [test] fn test_ord () { let mut a : SmallVec < u32 , 2 > = SmallVec :: new () ; let mut b : SmallVec < u32 , 2 > = SmallVec :: new () ; let mut c : SmallVec < u32 , 2 > = SmallVec :: new () ; a . push (1) ; b . push (1) ; b . push (1) ; c . push (1) ; c . push (2) ; assert ! (a < b) ; assert ! (b > a) ; assert ! (b < c) ; assert ! (c > b) ; }
};
}
