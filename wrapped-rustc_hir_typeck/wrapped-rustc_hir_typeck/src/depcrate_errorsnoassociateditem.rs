// Generated macro for NoAssociatedItem (struct)
macro_rules! Depcrate_errorsNoAssociatedItem {
() => {
// Module: crate::errors
// Provides: {"NoAssociatedItem"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_no_associated_item , code = E0599)] pub (crate) struct NoAssociatedItem < 'tcx > { # [primary_span] pub span : Span , pub item_kind : & 'static str , pub item_ident : Ident , pub ty_prefix : Cow < 'static , str > , pub ty : Ty < 'tcx > , pub trait_missing_method : bool , }
};
}
