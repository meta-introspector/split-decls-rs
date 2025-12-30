// Generated macro for poll_option_result_error_type (function)
macro_rules! Depcrate_matches_try_errpoll_option_result_error_type {
() => {
// Module: crate::matches::try_err
// Provides: {"poll_option_result_error_type"}
// Dependencies: {}
# [doc = " Extracts the error type from Poll<Option<Result<T, E>>>."] fn poll_option_result_error_type < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { if let ty :: Adt (def , subst) = ty . kind () && cx . tcx . lang_items () . get (LangItem :: Poll) == Some (def . did ()) && let ready_ty = subst . type_at (0) && let Some (some_ty) = option_arg_ty (cx , ready_ty) { result_error_type (cx , some_ty) } else { None } }
};
}
