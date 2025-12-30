// Generated macro for ty_allowed_without_raw_pointer_heuristic (function)
macro_rules! Depcrate_non_send_fields_in_send_tyty_allowed_without_raw_pointer_heuristic {
() => {
// Module: crate::non_send_fields_in_send_ty
// Provides: {"ty_allowed_without_raw_pointer_heuristic"}
// Dependencies: {}
# [doc = " Be more strict when the heuristic is disabled"] fn ty_allowed_without_raw_pointer_heuristic < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , send_trait : DefId) -> bool { if implements_trait (cx , ty , send_trait , & []) { return true ; } if is_copy (cx , ty) && ! contains_pointer_like (cx , ty) { return true ; } false }
};
}
