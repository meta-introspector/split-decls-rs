// Generated macro for stop_exploring (function)
macro_rules! Depcrate_rtstop_exploring {
() => {
// Module: crate::rt
// Provides: {"stop_exploring"}
// Dependencies: {}
# [doc = " Tells loom to stop exploring possible concurrent executions starting at this"] # [doc = " point."] # [doc = ""] # [doc = " Exploration can be enabled again with `explore`."] pub fn stop_exploring () { execution (| execution | { execution . path . critical () ; }) }
};
}
