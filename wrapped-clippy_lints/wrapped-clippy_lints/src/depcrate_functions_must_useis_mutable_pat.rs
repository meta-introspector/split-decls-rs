// Generated macro for is_mutable_pat (function)
macro_rules! Depcrate_functions_must_useis_mutable_pat {
() => {
// Module: crate::functions::must_use
// Provides: {"is_mutable_pat"}
// Dependencies: {}
fn is_mutable_pat (cx : & LateContext < '_ > , pat : & hir :: Pat < '_ > , tys : & mut DefIdSet) -> bool { if let hir :: PatKind :: Wild = pat . kind { return false ; } if cx . tcx . has_typeck_results (pat . hir_id . owner . def_id) { is_mutable_ty (cx , cx . tcx . typeck (pat . hir_id . owner . def_id) . pat_ty (pat) , tys) } else { false } }
};
}
