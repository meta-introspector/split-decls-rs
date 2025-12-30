// Generated macro for is_zero (function)
macro_rules! Depcrate_floating_point_arithmeticis_zero {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"is_zero"}
// Dependencies: {}
# [doc = " Returns true iff expr is some zero literal"] fn is_zero (cx : & LateContext < '_ > , expr : & Expr < '_ > , ctxt : SyntaxContext) -> bool { match ConstEvalCtxt :: new (cx) . eval_local (expr , ctxt) { Some (Int (i)) => i == 0 , Some (F32 (f)) => f == 0.0 , Some (F64 (f)) => f == 0.0 , _ => false , } }
};
}
