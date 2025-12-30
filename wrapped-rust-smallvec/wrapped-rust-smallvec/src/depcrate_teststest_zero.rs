// Generated macro for test_zero (function)
macro_rules! Depcrate_teststest_zero {
() => {
// Module: crate::tests
// Provides: {"test_zero"}
// Dependencies: {}
# [test] pub fn test_zero () { let mut v = SmallVec :: < _ , 0 > :: new () ; assert ! (! v . spilled ()) ; v . push (0usize) ; assert ! (v . spilled ()) ; assert_eq ! (&* v , & [0]) ; }
};
}
