// Generated macro for impl_176 (impl)
macro_rules! Depcrate_utils_wakers_vec_wakerimpl_176 {
() => {
// Module: crate::utils::wakers::vec::waker
// Provides: {"impl_176"}
// Dependencies: {}
impl Wake for InlineWakerVec { fn wake (self : Arc < Self >) { let mut readiness = self . readiness . lock () . unwrap () ; if ! readiness . set_ready (self . id) { readiness . parent_waker () . expect ("`parent_waker` not available from `Readiness`. Did you forget to call `Readiness::set_waker`?") . wake_by_ref () } } }
};
}
