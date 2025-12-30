// Generated macro for test_to_vec (function)
macro_rules! Depcrate_tests_mutable_settest_to_vec {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_to_vec"}
// Dependencies: {}
# [test] fn test_to_vec () { let strs = [NSMutableString :: from_str ("one") , NSMutableString :: from_str ("two") , NSMutableString :: from_str ("three") ,] ; let set = NSMutableSet :: from_retained_slice (& strs) ; let vec = set . to_vec () ; for str in & vec { str . appendString (ns_string ! (" times zero is zero")) ; } assert_eq ! (vec . len () , 3) ; assert ! (vec . iter () . all (| str | str . hasSuffix (ns_string ! ("zero")))) ; }
};
}
