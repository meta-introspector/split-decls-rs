// Generated macro for test_as_ref_path (function)
macro_rules! Depcrate_teststest_as_ref_path {
() => {
// Module: crate::tests
// Provides: {"test_as_ref_path"}
// Dependencies: {}
# [test] fn test_as_ref_path () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let s = CompactString :: new (short) ; assert_eq ! (AsRef ::< std :: path :: Path >:: as_ref (& s) . to_str () . unwrap () , short) ; let l = CompactString :: new (long) ; assert_eq ! (AsRef ::< std :: path :: Path >:: as_ref (& l) . to_str () . unwrap () , long) ; }
};
}
