// Generated macro for disable_incr_cache (function)
macro_rules! Depcrate_driver_aotdisable_incr_cache {
() => {
// Module: crate::driver::aot
// Provides: {"disable_incr_cache"}
// Dependencies: {}
fn disable_incr_cache () -> bool { env :: var ("CG_CLIF_DISABLE_INCR_CACHE") . as_deref () == Ok ("1") }
};
}
