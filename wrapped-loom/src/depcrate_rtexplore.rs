// Generated macro for explore (function)
macro_rules! Depcrate_rtexplore {
() => {
// Module: crate::rt
// Provides: {"explore"}
// Dependencies: {}
# [doc = " Tells loom to explore possible concurrent executions starting at this point."] pub fn explore () { execution (| execution | { execution . path . explore_state () ; }) }
};
}
