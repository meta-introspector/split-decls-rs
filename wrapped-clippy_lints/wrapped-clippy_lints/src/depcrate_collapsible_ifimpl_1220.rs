// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_collapsible_ifimpl_1220 {
() => {
// Module: crate::collapsible_if
// Provides: {"impl_1220"}
// Dependencies: {}
impl LateLintPass < '_ > for CollapsibleIf { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: If (cond , then , else_) = & expr . kind && ! expr . span . from_expansion () { if let Some (else_) = else_ && let ExprKind :: Block (else_ , None) = else_ . kind { self . check_collapsible_else_if (cx , then . span , else_) ; } else if else_ . is_none () && self . eligible_condition (cx , cond) && let ExprKind :: Block (then , None) = then . kind { self . check_collapsible_if_if (cx , expr , cond , then) ; } } } }
};
}
