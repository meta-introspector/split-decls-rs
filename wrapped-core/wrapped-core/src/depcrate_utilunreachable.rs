// Generated macro for unreachable (function)
macro_rules! Depcrate_utilunreachable {
() => {
// Module: crate::util
// Provides: {"unreachable"}
// Dependencies: {}
# [inline] unsafe fn unreachable () -> ! { if cfg ! (debug_assertions) { unreachable ! () ; } else { core :: hint :: unreachable_unchecked () } }
};
}
