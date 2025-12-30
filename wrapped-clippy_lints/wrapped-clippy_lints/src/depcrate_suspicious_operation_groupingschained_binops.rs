// Generated macro for chained_binops (function)
macro_rules! Depcrate_suspicious_operation_groupingschained_binops {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"chained_binops"}
// Dependencies: {}
fn chained_binops (kind : & ExprKind) -> Option < Vec < BinaryOp < '_ > > > { match kind { ExprKind :: Binary (_ , left_outer , right_outer) => chained_binops_helper (left_outer , right_outer) , ExprKind :: Paren (e) | ExprKind :: Unary (_ , e) => chained_binops (& e . kind) , _ => None , } }
};
}
