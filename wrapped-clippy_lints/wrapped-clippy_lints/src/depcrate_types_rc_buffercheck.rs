// Generated macro for check (function)
macro_rules! Depcrate_types_rc_buffercheck {
() => {
// Module: crate::types::rc_buffer
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , hir_ty : & Ty < '_ > , qpath : & QPath < '_ > , def_id : DefId) -> bool { let mut app = Applicability :: Unspecified ; let rc = match cx . tcx . get_diagnostic_name (def_id) { Some (sym :: Rc) => "Rc" , Some (sym :: Arc) => "Arc" , _ => return false , } ; if let Some (ty) = qpath_generic_tys (qpath) . next () && let Some (alternate) = match_buffer_type (cx , ty , & mut app) { span_lint_and_then (cx , RC_BUFFER , hir_ty . span , format ! ("usage of `{rc}<T>` when `T` is a buffer type") , | diag | { diag . span_suggestion_verbose (ty . span , "try" , alternate , app) ; } ,) ; true } else { false } }
};
}
