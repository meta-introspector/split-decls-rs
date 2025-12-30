// Generated macro for impl_21 (impl)
macro_rules! Depcrate_parking_lotimpl_21 {
() => {
// Module: crate::parking_lot
// Provides: {"impl_21"}
// Dependencies: {}
impl Drop for ThreadData { fn drop (& mut self) { NUM_THREADS . fetch_sub (1 , Ordering :: Relaxed) ; } }
};
}
