// Generated macro for test_threaded (function)
macro_rules! Depcrate_tests_mutable_arraytest_threaded {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_threaded"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] # [cfg (feature = "std")] fn test_threaded () { std :: thread :: scope (| s | { s . spawn (| | { let _ = NSMutableArray :: from_retained_slice (& [NSObject :: new () , NSObject :: new ()]) ; }) ; s . spawn (| | { let array = < NSMutableArray < NSObject > > :: alloc () ; let ptr = Allocated :: as_ptr (& array) ; assert ! (! ptr . is_null ()) ; }) ; }) ; }
};
}
