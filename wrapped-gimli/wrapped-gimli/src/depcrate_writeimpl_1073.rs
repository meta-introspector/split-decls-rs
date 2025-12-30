// Generated macro for impl_1073 (impl)
macro_rules! Depcrate_writeimpl_1073 {
() => {
// Module: crate::write
// Provides: {"impl_1073"}
// Dependencies: {}
# [cfg (debug_assertions)] impl Default for BaseId { fn default () -> Self { use std :: sync :: atomic ; static BASE_ID : atomic :: AtomicUsize = atomic :: AtomicUsize :: new (0) ; BaseId (BASE_ID . fetch_add (1 , atomic :: Ordering :: Relaxed)) } }
};
}
