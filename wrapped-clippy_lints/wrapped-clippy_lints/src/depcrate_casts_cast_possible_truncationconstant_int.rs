// Generated macro for constant_int (function)
macro_rules! Depcrate_casts_cast_possible_truncationconstant_int {
() => {
// Module: crate::casts::cast_possible_truncation
// Provides: {"constant_int"}
// Dependencies: {}
fn constant_int (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < u128 > { if let Some (Constant :: Int (c)) = ConstEvalCtxt :: new (cx) . eval (expr) { Some (c) } else { None } }
};
}
