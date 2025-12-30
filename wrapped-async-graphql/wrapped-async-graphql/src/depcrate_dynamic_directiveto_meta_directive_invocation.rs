// Generated macro for to_meta_directive_invocation (function)
macro_rules! Depcrate_dynamic_directiveto_meta_directive_invocation {
() => {
// Module: crate::dynamic::directive
// Provides: {"to_meta_directive_invocation"}
// Dependencies: {}
pub fn to_meta_directive_invocation (directives : Vec < Directive >) -> Vec < MetaDirectiveInvocation > { directives . into_iter () . map (MetaDirectiveInvocation :: from) . collect () }
};
}
