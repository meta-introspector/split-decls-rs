// Generated macro for impl_2592 (impl)
macro_rules! Depcrate_lock_bilockimpl_2592 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2592"}
// Dependencies: {}
impl < T : Unpin > Inner < T > { unsafe fn into_value (mut self) -> T { self . value . take () . unwrap () . into_inner () } }
};
}
