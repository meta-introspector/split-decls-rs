// Generated macro for test_inline (function)
macro_rules! Depcrate_teststest_inline {
() => {
// Module: crate::tests
// Provides: {"test_inline"}
// Dependencies: {}
# [test] pub fn test_inline () { let mut v = SmallVec :: < _ , 16 > :: new () ; v . push ("hello" . to_owned ()) ; v . push ("there" . to_owned ()) ; assert_eq ! (&* v , & ["hello" . to_owned () , "there" . to_owned () ,] [..]) ; }
};
}
