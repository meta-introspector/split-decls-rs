// Generated macro for impl_246 (impl)
macro_rules! Depcrate_progress_utilsimpl_246 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_246"}
// Dependencies: {}
impl < T > Count for DoOrDiscard < T > where T : Count , { fn set (& self , step : usize) { self . 0 . set (step) } fn step (& self) -> usize { self . 0 . step () } fn inc_by (& self , step : usize) { self . 0 . inc_by (step) } fn counter (& self) -> StepShared { self . 0 . counter () } }
};
}
