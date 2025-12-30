// Generated macro for impl_4143 (impl)
macro_rules! Depcrate_manual_clampimpl_4143 {
() => {
// Module: crate::manual_clamp
// Provides: {"impl_4143"}
// Dependencies: {}
impl < 'tcx > BinaryOp < 'tcx > { fn new (e : & 'tcx Expr < 'tcx >) -> Option < BinaryOp < 'tcx > > { match & e . kind { ExprKind :: Binary (op , left , right) => Some (BinaryOp { op : op . node , left , right , }) , _ => None , } } fn flip (& self) -> Self { Self { op : match self . op { BinOpKind :: Le => BinOpKind :: Ge , BinOpKind :: Lt => BinOpKind :: Gt , BinOpKind :: Ge => BinOpKind :: Le , BinOpKind :: Gt => BinOpKind :: Lt , other => other , } , left : self . right , right : self . left , } } }
};
}
