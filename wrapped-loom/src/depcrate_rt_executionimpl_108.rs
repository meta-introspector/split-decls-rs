// Generated macro for impl_108 (impl)
macro_rules! Depcrate_rt_executionimpl_108 {
() => {
// Module: crate::rt::execution
// Provides: {"impl_108"}
// Dependencies: {}
impl Id { pub (crate) fn new () -> Id { use std :: sync :: atomic :: AtomicUsize ; use std :: sync :: atomic :: Ordering :: Relaxed ; static NEXT_ID : AtomicUsize = AtomicUsize :: new (46_413_762) ; let next = NEXT_ID . fetch_add (1 , Relaxed) ; Id (next) } }
};
}
