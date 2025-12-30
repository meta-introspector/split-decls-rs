// Generated macro for into_iter (function)
macro_rules! Depcrate_testsinto_iter {
() => {
// Module: crate::tests
// Provides: {"into_iter"}
// Dependencies: {}
# [test] fn into_iter () { let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; assert_eq ! (v . into_iter () . collect ::< Vec < _ >> () , & [3]) ; let mut v : SmallVec < u8 , 2 > = SmallVec :: new () ; v . push (3) ; v . push (4) ; v . push (5) ; assert_eq ! (v . into_iter () . collect ::< Vec < _ >> () , & [3 , 4 , 5]) ; }
};
}
