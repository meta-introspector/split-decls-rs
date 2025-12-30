// Generated macro for synchronize (function)
macro_rules! Depcrate_rtsynchronize {
() => {
// Module: crate::rt
// Provides: {"synchronize"}
// Dependencies: {}
fn synchronize < F , R > (f : F) -> R where F : FnOnce (& mut Execution) -> R , { execution (| execution | { execution . threads . active_causality_inc () ; trace ! ("synchronize") ; f (execution) }) }
};
}
