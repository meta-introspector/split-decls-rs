// Generated macro for MetaDirectiveInvocation (struct)
macro_rules! Depcrate_registryMetaDirectiveInvocation {
() => {
// Module: crate::registry
// Provides: {"MetaDirectiveInvocation"}
// Dependencies: {}
# [doc = " actual directive invocation on SDL definitions"] # [derive (Debug , Clone)] pub struct MetaDirectiveInvocation { # [doc = " name of directive to invoke"] pub name : String , # [doc = " actual arguments passed to directive"] pub args : IndexMap < String , Value > , }
};
}
