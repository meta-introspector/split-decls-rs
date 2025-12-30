// Generated macro for impl_133 (impl)
macro_rules! Depcrate_utils_wakers_array_wakerimpl_133 {
() => {
// Module: crate::utils::wakers::array::waker
// Provides: {"impl_133"}
// Dependencies: {}
impl < const N : usize > Wake for InlineWakerArray < N > { fn wake (self : Arc < Self >) { let mut readiness = self . readiness . lock () . unwrap () ; if ! readiness . set_ready (self . id) { readiness . parent_waker () . expect ("`parent_waker` not available from `Readiness`. Did you forget to call `Readiness::set_waker`?") . wake_by_ref () } } }
};
}
