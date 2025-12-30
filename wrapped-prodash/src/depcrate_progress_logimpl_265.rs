// Generated macro for impl_265 (impl)
macro_rules! Depcrate_progress_logimpl_265 {
() => {
// Module: crate::progress::log
// Provides: {"impl_265"}
// Dependencies: {}
impl Count for Log { fn set (& self , step : Step) { self . step . store (step , Ordering :: SeqCst) ; self . maybe_log () } fn step (& self) -> usize { self . step . load (Ordering :: Relaxed) } fn inc_by (& self , step : Step) { self . step . fetch_add (step , Ordering :: Relaxed) ; self . maybe_log () } fn counter (& self) -> StepShared { self . step . clone () } }
};
}
