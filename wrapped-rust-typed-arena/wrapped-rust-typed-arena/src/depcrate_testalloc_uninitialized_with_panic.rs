// Generated macro for alloc_uninitialized_with_panic (function)
macro_rules! Depcrate_testalloc_uninitialized_with_panic {
() => {
// Module: crate::test
// Provides: {"alloc_uninitialized_with_panic"}
// Dependencies: {}
# [doc = " Check nothing bad happens by panicking during initialization of borrowed slice."] # [test] fn alloc_uninitialized_with_panic () { struct Dropper (bool) ; impl Drop for Dropper { fn drop (& mut self) { if self . 0 { panic ! () ; } } } let mut reached_first_init = false ; panic :: catch_unwind (AssertUnwindSafe (| | unsafe { let arena : Arena < Dropper > = Arena :: new () ; arena . reserve_extend (2) ; let uninitialized = arena . uninitialized_array () ; assert ! ((* uninitialized) . len () >= 2) ; ptr :: write ((* uninitialized) [0] . as_mut_ptr () , Dropper (false)) ; reached_first_init = true ; panic ! ("To drop the arena") ; })) . unwrap_err () ; assert ! (reached_first_init) ; }
};
}
