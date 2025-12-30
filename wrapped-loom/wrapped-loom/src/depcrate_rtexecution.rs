// Generated macro for execution (function)
macro_rules! Depcrate_rtexecution {
() => {
// Module: crate::rt
// Provides: {"execution"}
// Dependencies: {}
pub (crate) fn execution < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { Scheduler :: with_execution (f) }
};
}
