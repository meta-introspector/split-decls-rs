// Generated macro for impl_1102 (impl)
macro_rules! Depcrate_recovery_congestionimpl_1102 {
() => {
// Module: crate::recovery::congestion
// Provides: {"impl_1102"}
// Dependencies: {}
impl SsThresh { fn get (& self) -> usize { self . ssthresh } fn startup_exit (& self) -> Option < StartupExit > { self . startup_exit } fn update (& mut self , ssthresh : usize , in_css : bool) { if self . startup_exit . is_none () { let reason = if in_css { StartupExitReason :: PersistentQueue } else { StartupExitReason :: Loss } ; self . startup_exit = Some (StartupExit :: new (ssthresh , None , reason)) ; } self . ssthresh = ssthresh ; } }
};
}
