// Generated macro for impl_8019 (impl)
macro_rules! Depcrate_needless_question_markimpl_8019 {
() => {
// Module: crate::needless_question_mark
// Provides: {"impl_8019"}
// Dependencies: {}
impl LateLintPass < '_ > for NeedlessQuestionMark { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & '_ Expr < '_ >) { if let ExprKind :: Ret (Some (e)) = expr . kind { check (cx , e) ; } } fn check_body (& mut self , cx : & LateContext < '_ > , body : & '_ Body < '_ >) { if let ExprKind :: Block (Block { expr : Some (Expr { kind : ExprKind :: DropTemps (async_body) , .. }) , .. } , _ ,) = body . value . kind { if let ExprKind :: Block (Block { expr : Some (expr) , .. } , ..) = async_body . kind { check (cx , expr . peel_blocks ()) ; } } else { check (cx , body . value . peel_blocks ()) ; } } }
};
}
