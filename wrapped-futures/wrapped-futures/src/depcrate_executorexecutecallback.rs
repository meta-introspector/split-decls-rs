// Generated macro for ExecuteCallback (trait)
macro_rules! Depcrate_executorExecuteCallback {
() => {
// Module: crate::executor
// Provides: {"ExecuteCallback"}
// Dependencies: {}
# [doc = " Essentially `Box<FnOnce() + Send>`, just as a trait."] pub trait ExecuteCallback : Send + 'static { # [allow (missing_docs)] fn call (self : Box < Self >) ; }
};
}
