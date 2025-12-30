// Generated macro for extract_local (function)
macro_rules! Depcrate_unwrapextract_local {
() => {
// Module: crate::unwrap
// Provides: {"extract_local"}
// Dependencies: {}
# [doc = " Extracts either a local used by itself ([`Local::Pure`]), or (one or more levels of) field"] # [doc = " access to a local ([`Local::WithFieldAccess`])"] fn extract_local (cx : & LateContext < '_ > , mut expr : & Expr < '_ >) -> Option < Local > { let span = expr . span ; let mut field_indices = vec ! [] ; while let ExprKind :: Field (recv , _) = expr . kind && let Some (field_idx) = cx . typeck_results () . opt_field_index (expr . hir_id) { field_indices . push (field_idx) ; expr = recv ; } if let Some (local_id) = expr . res_local_id () { if field_indices . is_empty () { Some (Local :: Pure { local_id }) } else { Some (Local :: WithFieldAccess { local_id , field_indices , span , }) } } else { None } }
};
}
