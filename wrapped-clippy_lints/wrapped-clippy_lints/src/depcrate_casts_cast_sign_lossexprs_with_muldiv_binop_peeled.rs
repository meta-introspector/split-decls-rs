// Generated macro for exprs_with_muldiv_binop_peeled (function)
macro_rules! Depcrate_casts_cast_sign_lossexprs_with_muldiv_binop_peeled {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"exprs_with_muldiv_binop_peeled"}
// Dependencies: {}
# [doc = " Peels binary operators such as [`BinOpKind::Mul`], [`BinOpKind::Div`] or [`BinOpKind::Rem`],"] # [doc = " where the result depends on:"] # [doc = ""] # [doc = " - the number of negative values in the entire expression, or"] # [doc = " - the number of negative values on the left hand side of the expression."] # [doc = ""] # [doc = " Ignores overflow."] # [doc = ""] # [doc = ""] # [doc = " Expressions using other operators are preserved, so we can try to evaluate them later."] fn exprs_with_muldiv_binop_peeled < 'e > (expr : & 'e Expr < '_ >) -> Vec < & 'e Expr < 'e > > { let mut res = vec ! [] ; for_each_expr_without_closures (expr , | sub_expr | -> ControlFlow < Infallible , Descend > { if let ExprKind :: Binary (op , lhs , _rhs) = sub_expr . kind { if matches ! (op . node , BinOpKind :: Mul | BinOpKind :: Div) { ControlFlow :: Continue (Descend :: Yes) } else if matches ! (op . node , BinOpKind :: Rem | BinOpKind :: Shr) { res . push (lhs) ; ControlFlow :: Continue (Descend :: No) } else { res . push (sub_expr) ; ControlFlow :: Continue (Descend :: No) } } else { res . push (sub_expr) ; ControlFlow :: Continue (Descend :: No) } }) ; res }
};
}
