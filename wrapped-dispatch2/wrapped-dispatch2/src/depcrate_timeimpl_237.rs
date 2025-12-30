// Generated macro for impl_237 (impl)
macro_rules! Depcrate_timeimpl_237 {
() => {
// Module: crate::time
// Provides: {"impl_237"}
// Dependencies: {}
impl DispatchTime { # [doc = " The current time."] # [doc (alias = "DISPATCH_TIME_NOW")] pub const NOW : Self = Self (0) ; # [doc = " A time in the distant future."] # [doc (alias = "DISPATCH_TIME_FOREVER")] pub const FOREVER : Self = Self (u64 :: MAX) ; # [doc = " TODO."] # [doc (alias = "DISPATCH_WALLTIME_NOW")] pub const WALLTIME_NOW : Self = Self (! 1) ; }
};
}
