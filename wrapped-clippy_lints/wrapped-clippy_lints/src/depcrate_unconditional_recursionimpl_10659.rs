// Generated macro for impl_10659 (impl)
macro_rules! Depcrate_unconditional_recursionimpl_10659 {
() => {
// Module: crate::unconditional_recursion
// Provides: {"impl_10659"}
// Dependencies: {}
impl < 'a , 'tcx > Visitor < 'tcx > for CheckCalls < 'a , 'tcx > where 'tcx : 'a , { type NestedFilter = nested_filter :: OnlyBodies ; type Result = ControlFlow < () > ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> ControlFlow < () > { walk_expr (self , expr) ? ; if let ExprKind :: Call (f , _) = expr . kind && let ExprKind :: Path (qpath) = f . kind && is_default_method_on_current_ty (self . cx . tcx , qpath , self . implemented_ty_id) && let Some (method_def_id) = f . res (self . cx) . opt_def_id () && let Some (trait_def_id) = self . cx . tcx . trait_of_assoc (method_def_id) && self . cx . tcx . is_diagnostic_item (sym :: Default , trait_def_id) { span_error (self . cx , self . method_span , expr) ; return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) } }
};
}
