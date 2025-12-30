// Generated macro for impl_9084 (impl)
macro_rules! Depcrate_redundant_elseimpl_9084 {
() => {
// Module: crate::redundant_else
// Provides: {"impl_9084"}
// Dependencies: {}
impl < 'ast > Visitor < 'ast > for BreakVisitor { fn visit_block (& mut self , block : & 'ast Block) { self . is_break = match block . stmts . as_slice () { [.. , last] => self . check_stmt (last) , _ => false , } ; } fn visit_expr (& mut self , expr : & 'ast Expr) { self . is_break = match expr . kind { ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) => true , ExprKind :: Match (_ , ref arms , _) => arms . iter () . all (| arm | arm . body . is_none () || arm . body . as_deref () . is_some_and (| body | self . check_expr (body))) , ExprKind :: If (_ , ref then , Some (ref els)) => self . check_block (then) && self . check_expr (els) , ExprKind :: If (_ , _ , None) | ExprKind :: While (..) | ExprKind :: ForLoop { .. } | ExprKind :: Loop (..) => false , _ => { walk_expr (self , expr) ; return ; } , } ; } }
};
}
