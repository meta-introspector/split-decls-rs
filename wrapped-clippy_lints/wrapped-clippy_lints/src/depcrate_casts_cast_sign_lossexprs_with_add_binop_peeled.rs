// Generated macro for exprs_with_add_binop_peeled (function)
macro_rules! Depcrate_casts_cast_sign_lossexprs_with_add_binop_peeled {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"exprs_with_add_binop_peeled"}
// Dependencies: {}
# [doc = " Peels binary operators such as [`BinOpKind::Add`], where the result depends on:"] # [doc = ""] # [doc = " - all the expressions being positive, or"] # [doc = " - all the expressions being negative."] # [doc = ""] # [doc = " Ignores overflow."] # [doc = ""] # [doc = " Expressions using other operators are preserved, so we can try to evaluate them later."] fn exprs_with_add_binop_peeled < 'e > (expr : & 'e Expr < '_ >) -> Vec < & 'e Expr < 'e > > { let mut res = vec ! [] ; for_each_expr_without_closures (expr , | sub_expr | -> ControlFlow < Infallible , Descend > { if let ExprKind :: Binary (op , _lhs , _rhs) = sub_expr . kind { if matches ! (op . node , BinOpKind :: Add) { ControlFlow :: Continue (Descend :: Yes) } else { res . push (sub_expr) ; ControlFlow :: Continue (Descend :: No) } } else { res . push (sub_expr) ; ControlFlow :: Continue (Descend :: No) } }) ; res }
};
}
