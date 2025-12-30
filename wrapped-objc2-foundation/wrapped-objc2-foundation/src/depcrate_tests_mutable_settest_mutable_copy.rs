// Generated macro for test_mutable_copy (function)
macro_rules! Depcrate_tests_mutable_settest_mutable_copy {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_mutable_copy"}
// Dependencies: {}
# [test] # [cfg (feature = "NSObject")] fn test_mutable_copy () { use crate :: { NSMutableCopying , NSSet } ; let set1 = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")]) ; let set2 = set1 . mutableCopy () ; set2 . addObject (ns_string ! ("four")) ; assert ! (set1 . isSubsetOfSet (& set2)) ; assert_ne ! (set1 . mutableCopy () , set2) ; }
};
}
