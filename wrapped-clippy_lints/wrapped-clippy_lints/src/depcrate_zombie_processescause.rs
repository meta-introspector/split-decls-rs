// Generated macro for Cause (enum)
macro_rules! Depcrate_zombie_processesCause {
() => {
// Module: crate::zombie_processes
// Provides: {"Cause"}
// Dependencies: {}
# [derive (Copy , Clone)] enum Cause { # [doc = " No call to `wait()` at all"] NeverWait , # [doc = " `wait()` call exists, but not all code paths definitely lead to one due to"] # [doc = " an early return"] EarlyReturn { wait_span : Span , return_span : Span } , # [doc = " `wait()` call exists in some if branches but not this one"] MissingWaitInBranch { wait_span : Span , branch_span : Span } , # [doc = " `wait()` call exists in an if/then branch but it is missing an else block"] MissingElse { wait_span : Span , if_span : Span } , }
};
}
