// Generated macro for skip_branch (function)
macro_rules! Depcrate_rtskip_branch {
() => {
// Module: crate::rt
// Provides: {"skip_branch"}
// Dependencies: {}
# [doc = " Tells loom to stop exploring possible concurrent execution starting at this"] # [doc = " point."] # [doc = ""] # [doc = " Unlike `stop_exploring`, exploration cannot be restarted by `explore`."] pub fn skip_branch () { execution (| execution | execution . path . skip_branch ()) }
};
}
