// Generated macro for BinaryOp (struct)
macro_rules! Depcrate_manual_clampBinaryOp {
() => {
// Module: crate::manual_clamp
// Provides: {"BinaryOp"}
// Dependencies: {}
# [doc = " `ExprKind::Binary` but more narrowly typed"] # [derive (Debug , Clone , Copy)] struct BinaryOp < 'tcx > { op : BinOpKind , left : & 'tcx Expr < 'tcx > , right : & 'tcx Expr < 'tcx > , }
};
}
