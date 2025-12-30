// Generated macro for impl_9085 (impl)
macro_rules! Depcrate_redundant_elseimpl_9085 {
() => {
// Module: crate::redundant_else
// Provides: {"impl_9085"}
// Dependencies: {}
impl BreakVisitor { fn check < T > (& mut self , item : T , visit : fn (& mut Self , T)) -> bool { visit (self , item) ; std :: mem :: replace (& mut self . is_break , false) } fn check_block (& mut self , block : & Block) -> bool { self . check (block , Self :: visit_block) } fn check_expr (& mut self , expr : & Expr) -> bool { self . check (expr , Self :: visit_expr) } fn check_stmt (& mut self , stmt : & Stmt) -> bool { self . check (stmt , Self :: visit_stmt) } }
};
}
