// Generated macro for into_iter_rev (function)
macro_rules! Depcrate_testsinto_iter_rev {
() => {
// Module: crate::tests
// Provides: {"into_iter_rev"}
// Dependencies: {}
# [test] fn into_iter_rev () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . into_iter () . rev () . collect ::< Vec < _ >> () , & [3]) ; let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; v . push (4) ; v . push (5) ; assert_eq ! (v . into_iter () . rev () . collect ::< Vec < _ >> () , & [5 , 4 , 3]) ; }
};
}
