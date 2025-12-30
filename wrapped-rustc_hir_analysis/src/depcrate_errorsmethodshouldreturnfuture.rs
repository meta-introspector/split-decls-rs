// Generated macro for MethodShouldReturnFuture (struct)
macro_rules! Depcrate_errorsMethodShouldReturnFuture {
() => {
// Module: crate::errors
// Provides: {"MethodShouldReturnFuture"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_method_should_return_future)] pub (crate) struct MethodShouldReturnFuture { # [primary_span] pub span : Span , pub method_name : Ident , # [note] pub trait_item_span : Option < Span > , }
};
}
