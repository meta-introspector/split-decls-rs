// Generated macro for ExecutionEndResult (enum)
macro_rules! Depcrate_concurrency_genmcExecutionEndResult {
() => {
// Module: crate::concurrency::genmc
// Provides: {"ExecutionEndResult"}
// Dependencies: {}
# [derive (Debug)] pub enum ExecutionEndResult { # [doc = " An error occurred at the end of the execution."] Error (String) , # [doc = " No errors occurred, and there are more executions to explore."] Continue , # [doc = " No errors occurred and we are finished."] Stop , }
};
}
