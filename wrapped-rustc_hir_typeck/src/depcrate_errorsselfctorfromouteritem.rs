// Generated macro for SelfCtorFromOuterItem (struct)
macro_rules! Depcrate_errorsSelfCtorFromOuterItem {
() => {
// Module: crate::errors
// Provides: {"SelfCtorFromOuterItem"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_self_ctor_from_outer_item , code = E0401)] pub (crate) struct SelfCtorFromOuterItem { # [primary_span] pub span : Span , # [label] pub impl_span : Span , # [subdiagnostic] pub sugg : Option < ReplaceWithName > , }
};
}
