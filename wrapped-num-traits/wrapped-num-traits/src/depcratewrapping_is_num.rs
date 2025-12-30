// Generated macro for wrapping_is_num (function)
macro_rules! Depcratewrapping_is_num {
() => {
// Module: crate
// Provides: {"wrapping_is_num"}
// Dependencies: {}
# [test] fn wrapping_is_num () { fn require_num < T : Num > (_ : & T) { } require_num (& Wrapping (42_u32)) ; require_num (& Wrapping (- 42)) ; }
};
}
