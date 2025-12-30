// Generated macro for Context (trait)
macro_rules! Depcrate_utilsContext {
() => {
// Module: crate::utils
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Add additional context to errors"] # [cfg (feature = "std")] pub (crate) trait Context { fn context (self , message : impl Fn () -> String) -> Self ; }
};
}
