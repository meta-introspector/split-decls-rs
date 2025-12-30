// Generated macro for impl_4303 (impl)
macro_rules! Depcrate_manual_main_separator_strimpl_4303 {
() => {
// Module: crate::manual_main_separator_str
// Provides: {"impl_4303"}
// Dependencies: {}
impl LateLintPass < '_ > for ManualMainSeparatorStr { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { let (target , _) = peel_hir_expr_refs (expr) ; if let ExprKind :: MethodCall (path , receiver , & [] , _) = target . kind && path . ident . name == sym :: to_string && let ExprKind :: Path (QPath :: Resolved (None , path)) = receiver . kind && let Res :: Def (DefKind :: Const , receiver_def_id) = path . res && cx . ty_based_def (target) . opt_parent (cx) . is_diag_item (cx , sym :: ToString) && cx . tcx . is_diagnostic_item (sym :: path_main_separator , receiver_def_id) && let ty :: Ref (_ , ty , Mutability :: Not) = cx . typeck_results () . expr_ty_adjusted (expr) . kind () && ty . is_str () && self . msrv . meets (cx , msrvs :: PATH_MAIN_SEPARATOR_STR) { span_lint_and_sugg (cx , MANUAL_MAIN_SEPARATOR_STR , expr . span , "taking a reference on `std::path::MAIN_SEPARATOR` conversion to `String`" , "replace with" , "std::path::MAIN_SEPARATOR_STR" . to_owned () , Applicability :: MachineApplicable ,) ; } } }
};
}
