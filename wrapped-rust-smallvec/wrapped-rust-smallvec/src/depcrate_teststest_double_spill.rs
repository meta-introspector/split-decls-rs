// Generated macro for test_double_spill (function)
macro_rules! Depcrate_teststest_double_spill {
() => {
// Module: crate::tests
// Provides: {"test_double_spill"}
// Dependencies: {}
# [test] pub fn test_double_spill () { let mut v = SmallVec :: < _ , 2 > :: new () ; v . push ("hello" . to_owned ()) ; v . push ("there" . to_owned ()) ; v . push ("burma" . to_owned ()) ; v . push ("shave" . to_owned ()) ; v . push ("hello" . to_owned ()) ; v . push ("there" . to_owned ()) ; v . push ("burma" . to_owned ()) ; v . push ("shave" . to_owned ()) ; assert_eq ! (&* v , & ["hello" . to_owned () , "there" . to_owned () , "burma" . to_owned () , "shave" . to_owned () , "hello" . to_owned () , "there" . to_owned () , "burma" . to_owned () , "shave" . to_owned () ,] [..]) ; }
};
}
