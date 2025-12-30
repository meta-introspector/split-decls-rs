// Generated macro for impl_116 (impl)
macro_rules! Depcrate_debt_listimpl_116 {
() => {
// Module: crate::debt::list
// Provides: {"impl_116"}
// Dependencies: {}
impl Drop for NodeReservation < '_ > { fn drop (& mut self) { self . 0 . active_writers . fetch_sub (1 , Release) ; } }
};
}
