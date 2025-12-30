// Generated macro for test_drop_panic_smallvec (function)
macro_rules! Depcrate_teststest_drop_panic_smallvec {
() => {
// Module: crate::tests
// Provides: {"test_drop_panic_smallvec"}
// Dependencies: {}
# [test] # [should_panic] fn test_drop_panic_smallvec () { struct DropPanic ; impl Drop for DropPanic { fn drop (& mut self) { panic ! ("drop") ; } } let mut v = SmallVec :: < _ , 1 > :: new () ; v . push (DropPanic) ; }
};
}
