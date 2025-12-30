// Generated macro for test_sort (function)
macro_rules! Depcrate_tests_mutable_arraytest_sort {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_sort"}
// Dependencies: {}
# [test] # [cfg (all (feature = "NSObjCRuntime" , feature = "NSString"))] fn test_sort () { use crate :: ns_string ; use alloc :: string :: ToString ; let strings = [ns_string ! ("hello") , ns_string ! ("hi")] ; let strings = NSMutableArray :: from_slice (& strings) ; strings . sort_by (| s1 , s2 | s1 . len () . cmp (& s2 . len ())) ; assert_eq ! (strings . objectAtIndex (0) . to_string () , "hi") ; assert_eq ! (strings . objectAtIndex (1) . to_string () , "hello") ; }
};
}
