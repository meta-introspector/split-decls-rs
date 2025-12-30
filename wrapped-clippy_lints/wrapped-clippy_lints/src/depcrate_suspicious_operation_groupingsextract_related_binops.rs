// Generated macro for extract_related_binops (function)
macro_rules! Depcrate_suspicious_operation_groupingsextract_related_binops {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"extract_related_binops"}
// Dependencies: {}
fn extract_related_binops (kind : & ExprKind) -> Option < Vec < BinaryOp < '_ > > > { append_opt_vecs (chained_binops (kind) , if_statement_binops (kind)) }
};
}
