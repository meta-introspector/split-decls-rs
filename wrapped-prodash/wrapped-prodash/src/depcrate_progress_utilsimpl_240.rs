// Generated macro for impl_240 (impl)
macro_rules! Depcrate_progress_utilsimpl_240 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_240"}
// Dependencies: {}
impl < L , R > Count for Either < L , R > where L : Count , R : Count , { fn set (& self , step : usize) { match self { Either :: Left (l) => l . set (step) , Either :: Right (r) => r . set (step) , } } fn step (& self) -> usize { match self { Either :: Left (l) => l . step () , Either :: Right (r) => r . step () , } } fn inc_by (& self , step : usize) { match self { Either :: Left (l) => l . inc_by (step) , Either :: Right (r) => r . inc_by (step) , } } fn counter (& self) -> StepShared { match self { Either :: Left (l) => l . counter () , Either :: Right (r) => r . counter () , } } }
};
}
