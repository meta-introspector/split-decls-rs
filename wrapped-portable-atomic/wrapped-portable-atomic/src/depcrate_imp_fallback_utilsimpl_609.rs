// Generated macro for impl_609 (impl)
macro_rules! Depcrate_imp_fallback_utilsimpl_609 {
() => {
// Module: crate::imp::fallback::utils
// Provides: {"impl_609"}
// Dependencies: {}
impl Backoff { # [inline] pub (crate) const fn new () -> Self { Self { step : 0 } } # [inline] pub (crate) fn snooze (& mut self) { if self . step <= SPIN_LIMIT { for _ in 0 .. 1 << self . step { # [allow (deprecated)] core :: sync :: atomic :: spin_loop_hint () ; } self . step += 1 ; } else { # [cfg (not (feature = "std"))] for _ in 0 .. 1 << self . step { # [allow (deprecated)] core :: sync :: atomic :: spin_loop_hint () ; } # [cfg (feature = "std")] std :: thread :: yield_now () ; } } }
};
}
