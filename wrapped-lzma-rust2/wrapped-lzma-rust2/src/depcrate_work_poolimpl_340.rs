// Generated macro for impl_340 (impl)
macro_rules! Depcrate_work_poolimpl_340 {
() => {
// Module: crate::work_pool
// Provides: {"impl_340"}
// Dependencies: {}
impl < W , R > Drop for WorkPool < W , R > { fn drop (& mut self) { self . shutdown_flag . store (true , Ordering :: Release) ; self . work_queue . close () ; } }
};
}
