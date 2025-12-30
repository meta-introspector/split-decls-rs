// Generated macro for check (function)
macro_rules! Depcrate_types_option_optioncheck {
() => {
// Module: crate::types::option_option
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , hir_ty : & hir :: Ty < '_ > , qpath : & QPath < '_ > , def_id : DefId) -> bool { if cx . tcx . is_diagnostic_item (sym :: Option , def_id) && let Some (arg) = qpath_generic_tys (qpath) . next () && arg . basic_res () . opt_def_id () == Some (def_id) { span_lint_and_then (cx , OPTION_OPTION , hir_ty . span , "use of `Option<Option<T>>`" , | diag | { let inner_opt = snippet (cx , arg . span , "_") ; diag . help (format ! ("consider using `{inner_opt}`, or a custom enum if you need to distinguish all 3 cases")) ; } ,) ; true } else { false } }
};
}
