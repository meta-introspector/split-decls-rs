// Generated macro for impl_8054 (impl)
macro_rules! Depcrate_neg_multiplyimpl_8054 {
() => {
// Module: crate::neg_multiply
// Provides: {"impl_8054"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NegMultiply { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if let ExprKind :: Binary (ref op , left , right) = e . kind && BinOpKind :: Mul == op . node { match (& left . kind , & right . kind) { (& ExprKind :: Unary (..) , & ExprKind :: Unary (..)) => { } , (& ExprKind :: Unary (UnOp :: Neg , lit) , _) => check_mul (cx , e , lit , right) , (_ , & ExprKind :: Unary (UnOp :: Neg , lit)) => check_mul (cx , e , lit , left) , _ => { } , } } } }
};
}
