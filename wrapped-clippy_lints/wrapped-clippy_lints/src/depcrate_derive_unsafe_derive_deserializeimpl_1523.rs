// Generated macro for impl_1523 (impl)
macro_rules! Depcrate_derive_unsafe_derive_deserializeimpl_1523 {
() => {
// Module: crate::derive::unsafe_derive_deserialize
// Provides: {"impl_1523"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for UnsafeVisitor < '_ , 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = nested_filter :: All ; fn visit_fn (& mut self , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , body_id : BodyId , _ : Span , id : LocalDefId ,) -> Self :: Result { if let Some (header) = kind . header () && header . is_unsafe () { ControlFlow :: Break (()) } else { walk_fn (self , kind , decl , body_id , id) } } fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) -> Self :: Result { if let ExprKind :: Block (block , _) = expr . kind && block . rules == BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) && block . span . source_callee () . and_then (| expr | expr . macro_def_id) . is_none_or (| did | ! self . cx . tcx . is_diagnostic_item (sym :: pin_macro , did)) { return ControlFlow :: Break (()) ; } walk_expr (self , expr) } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
