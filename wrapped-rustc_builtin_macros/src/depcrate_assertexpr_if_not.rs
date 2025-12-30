// Generated macro for expr_if_not (function)
macro_rules! Depcrate_assertexpr_if_not {
() => {
// Module: crate::assert
// Provides: {"expr_if_not"}
// Dependencies: {}
fn expr_if_not (cx : & ExtCtxt < '_ > , span : Span , cond : Box < Expr > , then : Box < Expr > , els : Option < Box < Expr > > ,) -> Box < Expr > { cx . expr_if (span , cx . expr (span , ExprKind :: Unary (UnOp :: Not , cond)) , then , els) }
};
}
