// Generated macro for wrapping_is_bounded (function)
macro_rules! Depcrate_boundswrapping_is_bounded {
() => {
// Module: crate::bounds
// Provides: {"wrapping_is_bounded"}
// Dependencies: {}
# [test] fn wrapping_is_bounded () { fn require_bounded < T : Bounded > (_ : & T) { } require_bounded (& Wrapping (42_u32)) ; require_bounded (& Wrapping (- 42)) ; }
};
}
