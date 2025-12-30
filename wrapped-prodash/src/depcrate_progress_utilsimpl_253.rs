// Generated macro for impl_253 (impl)
macro_rules! Depcrate_progress_utilsimpl_253 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_253"}
// Dependencies: {}
impl < T : NestedProgress > Count for ThroughputOnDrop < T > { fn set (& self , step : usize) { self . 0 . set (step) } fn step (& self) -> usize { self . 0 . step () } fn inc_by (& self , step : usize) { self . 0 . inc_by (step) } fn counter (& self) -> StepShared { self . 0 . counter () } }
};
}
