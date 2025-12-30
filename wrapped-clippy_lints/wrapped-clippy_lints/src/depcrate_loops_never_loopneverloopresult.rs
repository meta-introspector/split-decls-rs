// Generated macro for NeverLoopResult (enum)
macro_rules! Depcrate_loops_never_loopNeverLoopResult {
() => {
// Module: crate::loops::never_loop
// Provides: {"NeverLoopResult"}
// Dependencies: {}
# [doc = " The `never_loop` analysis keeps track of three things:"] # [doc = ""] # [doc = " * Has any (reachable) code path hit a `continue` of the main loop?"] # [doc = " * Is the current code path diverging (that is, the next expression is not reachable)"] # [doc = " * For each block label `'a` inside the main loop, has any (reachable) code path encountered a"] # [doc = "   `break 'a`?"] # [doc = ""] # [doc = " The first two bits of information are in this enum, and the last part is in the"] # [doc = " `local_labels` variable, which contains a list of `(block_id, reachable)` pairs ordered by"] # [doc = " scope."] # [derive (Clone , Debug)] enum NeverLoopResult { # [doc = " A continue may occur for the main loop."] MayContinueMainLoop , # [doc = " We have not encountered any main loop continue,"] # [doc = " but we are diverging (subsequent control flow is not reachable)"] Diverging { break_spans : Vec < Span > , never_spans : Vec < Span > , } , # [doc = " We have not encountered any main loop continue,"] # [doc = " and subsequent control flow is (possibly) reachable"] Normal , }
};
}
