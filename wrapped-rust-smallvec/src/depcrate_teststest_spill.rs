// Generated macro for test_spill (function)
macro_rules! Depcrate_teststest_spill {
() => {
// Module: crate::tests
// Provides: {"test_spill"}
// Dependencies: {}
# [test] pub fn test_spill () { let mut v = SmallVec :: < _ , 2 > :: new () ; v . push ("hello" . to_owned ()) ; assert_eq ! (v [0] , "hello") ; v . push ("there" . to_owned ()) ; v . push ("burma" . to_owned ()) ; assert_eq ! (v [0] , "hello") ; v . push ("shave" . to_owned ()) ; assert_eq ! (&* v , & ["hello" . to_owned () , "there" . to_owned () , "burma" . to_owned () , "shave" . to_owned () ,] [..]) ; }
};
}
