// Generated macro for SelfCtorFromOuterItemLint (struct)
macro_rules! Depcrate_errorsSelfCtorFromOuterItemLint {
() => {
// Module: crate::errors
// Provides: {"SelfCtorFromOuterItemLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_typeck_self_ctor_from_outer_item)] pub (crate) struct SelfCtorFromOuterItemLint { # [label] pub impl_span : Span , # [subdiagnostic] pub sugg : Option < ReplaceWithName > , }
};
}
