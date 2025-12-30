// Generated macro for test_copy (function)
macro_rules! Depcrate_tests_settest_copy {
() => {
// Module: crate::tests::set
// Provides: {"test_copy"}
// Dependencies: {}
# [test] # [cfg (feature = "NSObject")] fn test_copy () { use crate :: NSCopying ; let set1 = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")]) ; let set2 = set1 . copy () ; assert_eq ! (set1 , set2) ; }
};
}
