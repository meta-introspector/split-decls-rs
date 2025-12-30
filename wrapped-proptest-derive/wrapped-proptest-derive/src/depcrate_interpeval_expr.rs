// Generated macro for eval_expr (function)
macro_rules! Depcrate_interpeval_expr {
() => {
// Module: crate::interp
// Provides: {"eval_expr"}
// Dependencies: {}
# [doc = " A **very** simple CTFE interpreter for some basic arithmetic:"] pub fn eval_expr (expr : & E) -> Option < u128 > { match expr { E :: Lit (expr) => eval_lit (expr) , E :: Binary (expr) => eval_binary (expr) , E :: Unary (expr) => eval_unary (expr) , E :: Paren (expr) => eval_expr (& expr . expr) , E :: Group (expr) => eval_expr (& expr . expr) , _ => None , } }
};
}
