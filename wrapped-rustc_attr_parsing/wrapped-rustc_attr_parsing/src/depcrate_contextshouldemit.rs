// Generated macro for ShouldEmit (enum)
macro_rules! Depcrate_contextShouldEmit {
() => {
// Module: crate::context
// Provides: {"ShouldEmit"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub enum ShouldEmit { # [doc = " The operations will emit errors, and lints, and errors are fatal."] # [doc = ""] # [doc = " Only relevant when early parsing, in late parsing equivalent to `ErrorsAndLints`."] # [doc = " Late parsing is never fatal, and instead tries to emit as many diagnostics as possible."] EarlyFatal { also_emit_lints : bool } , # [doc = " The operation will emit errors and lints."] # [doc = " This is usually what you need."] ErrorsAndLints , # [doc = " The operation will emit *not* errors and lints."] # [doc = " Use this if you are *sure* that this operation will be called at a different time with `ShouldEmit::ErrorsAndLints`."] Nothing , }
};
}
