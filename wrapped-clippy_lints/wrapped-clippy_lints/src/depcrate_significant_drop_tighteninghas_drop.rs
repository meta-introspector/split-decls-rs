// Generated macro for has_drop (function)
macro_rules! Depcrate_significant_drop_tighteninghas_drop {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"has_drop"}
// Dependencies: {}
fn has_drop (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , first_bind_ident : Option < Ident >) -> bool { if let hir :: ExprKind :: Call (fun , [first_arg]) = expr . kind && let hir :: ExprKind :: Path (hir :: QPath :: Resolved (_ , fun_path)) = & fun . kind && let Res :: Def (DefKind :: Fn , did) = fun_path . res && cx . tcx . is_diagnostic_item (sym :: mem_drop , did) { let has_ident = | local_expr : & hir :: Expr < '_ > | { if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (_ , arg_path)) = & local_expr . kind && let [first_arg_ps , ..] = arg_path . segments && let Some (first_bind_ident) = first_bind_ident && first_arg_ps . ident == first_bind_ident { true } else { false } } ; if has_ident (first_arg) { return true ; } if let hir :: ExprKind :: Tup (value) = & first_arg . kind && value . iter () . any (has_ident) { return true ; } } false }
};
}
