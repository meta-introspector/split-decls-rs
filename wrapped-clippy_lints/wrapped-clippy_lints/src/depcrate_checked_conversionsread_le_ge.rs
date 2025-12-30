// Generated macro for read_le_ge (function)
macro_rules! Depcrate_checked_conversionsread_le_ge {
() => {
// Module: crate::checked_conversions
// Provides: {"read_le_ge"}
// Dependencies: {}
# [doc = " Attempts to read either `<=` or `>=` with a normalized operand order."] fn read_le_ge < 'tcx > (op : BinOpKind , lhs : & 'tcx Expr < 'tcx > , rhs : & 'tcx Expr < 'tcx > ,) -> Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx >) > { match op { BinOpKind :: Le => Some ((lhs , rhs)) , BinOpKind :: Ge => Some ((rhs , lhs)) , _ => None , } }
};
}
