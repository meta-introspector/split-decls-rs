// Generated macro for expr_muldiv_sign (function)
macro_rules! Depcrate_casts_cast_sign_lossexpr_muldiv_sign {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"expr_muldiv_sign"}
// Dependencies: {}
# [doc = " Peels binary operators such as [`BinOpKind::Mul`] or [`BinOpKind::Rem`],"] # [doc = " where the result could always be positive. See [`exprs_with_muldiv_binop_peeled()`] for details."] # [doc = ""] # [doc = " Returns the sign of the list of peeled expressions."] fn expr_muldiv_sign (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Sign { let mut negative_count = 0 ; let exprs = exprs_with_muldiv_binop_peeled (expr) ; for expr in exprs { match expr_sign (cx , expr , None) { Sign :: Negative => negative_count += 1 , Sign :: Uncertain => return Sign :: Uncertain , Sign :: ZeroOrPositive => () , } } if negative_count % 2 == 1 { Sign :: Negative } else { Sign :: ZeroOrPositive } }
};
}
