// Generated macro for test_hash (function)
macro_rules! Depcrate_tests_stringtest_hash {
() => {
// Module: crate::tests::string
// Provides: {"test_hash"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn test_hash () { use core :: hash :: Hash ; use core :: hash :: Hasher ; use std :: collections :: hash_map :: DefaultHasher ; let s1 = NSString :: from_str ("example string goes here") ; let s2 = NSString :: from_str ("example string goes here") ; let mut hashstate = DefaultHasher :: new () ; let mut hashstate2 = DefaultHasher :: new () ; s1 . hash (& mut hashstate) ; s2 . hash (& mut hashstate2) ; assert_eq ! (hashstate . finish () , hashstate2 . finish ()) ; }
};
}
