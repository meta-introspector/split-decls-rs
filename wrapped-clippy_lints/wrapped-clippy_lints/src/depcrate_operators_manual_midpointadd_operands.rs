// Generated macro for add_operands (function)
macro_rules! Depcrate_operators_manual_midpointadd_operands {
() => {
// Module: crate::operators::manual_midpoint
// Provides: {"add_operands"}
// Dependencies: {}
# [doc = " Return the left and right operands if `expr` represents an addition"] fn add_operands < 'e , 'tcx > (expr : & 'e Expr < 'tcx >) -> Option < (& 'e Expr < 'tcx > , & 'e Expr < 'tcx >) > { match expr . kind { ExprKind :: Binary (op , left , right) if op . node == BinOpKind :: Add => Some ((left , right)) , _ => None , } }
};
}
