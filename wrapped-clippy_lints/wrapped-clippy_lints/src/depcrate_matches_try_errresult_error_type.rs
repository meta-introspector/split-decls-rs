// Generated macro for result_error_type (function)
macro_rules! Depcrate_matches_try_errresult_error_type {
() => {
// Module: crate::matches::try_err
// Provides: {"result_error_type"}
// Dependencies: {}
# [doc = " Extracts the error type from Result<T, E>."] fn result_error_type < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { if let ty :: Adt (def , subst) = ty . kind () && cx . tcx . is_diagnostic_item (sym :: Result , def . did ()) { Some (subst . type_at (1)) } else { None } }
};
}
