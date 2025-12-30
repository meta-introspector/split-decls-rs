// Generated macro for busy_wait (function)
macro_rules! Depcrate_syncbusy_wait {
() => {
// Module: crate::sync
// Provides: {"busy_wait"}
// Dependencies: {}
# [doc = " Notify the CPU that we are currently busy-waiting."] # [inline] pub (crate) fn busy_wait () { # [cfg (feature = "std")] yield_now () ; # [cfg (not (feature = "std"))] spin_loop () ; }
};
}
