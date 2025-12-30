// Generated macro for impl_238 (impl)
macro_rules! Depcrate_danglingimpl_238 {
() => {
// Module: crate::dangling
// Provides: {"impl_238"}
// Dependencies: {}
impl Visitor < '_ > for DanglingPointerSearcher < '_ , '_ > { fn visit_expr (& mut self , expr : & Expr < '_ >) -> Self :: Result { if ! self . inside_call_args { lint_expr (self . cx , expr) } match expr . kind { ExprKind :: Call (lhs , args) | ExprKind :: MethodCall (_ , lhs , args , _) => { self . visit_expr (lhs) ; self . with_inside_call_args (true , | this | walk_list ! (this , visit_expr , args)) } ExprKind :: Block (& Block { stmts , expr , .. } , _) => { self . with_inside_call_args (false , | this | walk_list ! (this , visit_stmt , stmts)) ; visit_opt ! (self , visit_expr , expr) } _ => walk_expr (self , expr) , } } }
};
}
