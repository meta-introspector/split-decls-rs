// Generated macro for TypeNoCopy (enum)
macro_rules! Depcrate_session_diagnosticsTypeNoCopy {
() => {
// Module: crate::session_diagnostics
// Provides: {"TypeNoCopy"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum TypeNoCopy < 'a , 'tcx > { # [label (borrowck_ty_no_impl_copy)] Label { is_partial_move : bool , ty : Ty < 'tcx > , place : & 'a str , # [primary_span] span : Span , } , # [note (borrowck_ty_no_impl_copy)] Note { is_partial_move : bool , ty : Ty < 'tcx > , place : & 'a str } , }
};
}
