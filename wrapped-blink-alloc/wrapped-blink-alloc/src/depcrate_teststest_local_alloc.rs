// Generated macro for test_local_alloc (function)
macro_rules! Depcrate_teststest_local_alloc {
() => {
// Module: crate::tests
// Provides: {"test_local_alloc"}
// Dependencies: {}
# [test] fn test_local_alloc () { let mut blink = BlinkAlloc :: new () ; let ptr = blink . allocate (Layout :: new :: < usize > ()) . unwrap () . cast :: < usize > () ; unsafe { core :: ptr :: write (ptr . as_ptr () , 42) ; } blink . reset () ; }
};
}
