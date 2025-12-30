// Generated macro for unexpanded_binop_operands (function)
macro_rules! Depcrate_manual_is_power_of_twounexpanded_binop_operands {
() => {
// Module: crate::manual_is_power_of_two
// Provides: {"unexpanded_binop_operands"}
// Dependencies: {}
# [doc = " Return the operands of the `expr` binary operation if the operator is `op` and none of the"] # [doc = " operands come from expansion."] fn unexpanded_binop_operands < 'hir > (expr : & Expr < 'hir > , op : BinOpKind) -> Option < (& 'hir Expr < 'hir > , & 'hir Expr < 'hir >) > { if let ExprKind :: Binary (binop , lhs , rhs) = expr . kind && binop . node == op && ! lhs . span . from_expansion () && ! rhs . span . from_expansion () { Some ((lhs , rhs)) } else { None } }
};
}
