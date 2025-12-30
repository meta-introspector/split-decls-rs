// Generated macro for expr_add_sign (function)
macro_rules! Depcrate_casts_cast_sign_lossexpr_add_sign {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"expr_add_sign"}
// Dependencies: {}
# [doc = " Peels binary operators such as [`BinOpKind::Add`], where the result could always be positive."] # [doc = " See [`exprs_with_add_binop_peeled()`] for details."] # [doc = ""] # [doc = " Returns the sign of the list of peeled expressions."] fn expr_add_sign (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Sign { let mut negative_count = 0 ; let mut positive_count = 0 ; let exprs = exprs_with_add_binop_peeled (expr) ; for expr in exprs { match expr_sign (cx , expr , None) { Sign :: Negative => negative_count += 1 , Sign :: Uncertain => return Sign :: Uncertain , Sign :: ZeroOrPositive => positive_count += 1 , } } if negative_count == 0 { Sign :: ZeroOrPositive } else if positive_count == 0 { Sign :: Negative } else { Sign :: Uncertain } }
};
}
