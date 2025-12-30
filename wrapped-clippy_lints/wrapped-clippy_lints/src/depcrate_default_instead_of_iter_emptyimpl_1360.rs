// Generated macro for impl_1360 (impl)
macro_rules! Depcrate_default_instead_of_iter_emptyimpl_1360 {
() => {
// Module: crate::default_instead_of_iter_empty
// Provides: {"impl_1360"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DefaultIterEmpty { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Call (iter_expr , []) = & expr . kind && let ExprKind :: Path (QPath :: TypeRelative (ty , _)) = & iter_expr . kind && let TyKind :: Path (ty_path) = & ty . kind && let QPath :: Resolved (None , path) = ty_path && let def :: Res :: Def (_ , def_id) = & path . res && cx . tcx . is_diagnostic_item (sym :: IterEmpty , * def_id) && let ctxt = expr . span . ctxt () && ty . span . ctxt () == ctxt { let mut applicability = Applicability :: MachineApplicable ; let Some (path) = std_or_core (cx) else { return } ; let path = format ! ("{path}::iter::empty") ; let sugg = make_sugg (cx , ty_path , ctxt , & mut applicability , & path) ; span_lint_and_sugg (cx , DEFAULT_INSTEAD_OF_ITER_EMPTY , expr . span , format ! ("`{path}()` is the more idiomatic way") , "try" , sugg , applicability ,) ; } } }
};
}
