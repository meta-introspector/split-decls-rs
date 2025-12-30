// Generated macro for impl_1448 (impl)
macro_rules! Depcrate_status_iterimpl_1448 {
() => {
// Module: crate::status::iter
// Provides: {"impl_1448"}
// Dependencies: {}
# [cfg (feature = "parallel")] impl Drop for Iter { fn drop (& mut self) { crate :: util :: parallel_iter_drop (self . rx_and_join . take () , & self . should_interrupt) ; } }
};
}
