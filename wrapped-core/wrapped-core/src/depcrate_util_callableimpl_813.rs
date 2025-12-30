// Generated macro for impl_813 (impl)
macro_rules! Depcrate_util_callableimpl_813 {
() => {
// Module: crate::util::callable
// Provides: {"impl_813"}
// Dependencies: {}
impl FromMeta for Callable { fn from_expr (expr : & Expr) -> Result < Self > { match expr { Expr :: Path (_) | Expr :: Closure (_) => Ok (Self { call : expr . clone () }) , Expr :: Lit (ExprLit { lit : Lit :: Str (s) , .. }) => s . parse :: < Path > () . map_err (| e | { Error :: custom (format ! ("must be a path if it's a string: {}" , e)) . with_span (s) }) . map (Self :: from) , _ => Err (Error :: unexpected_expr_type (expr)) , } } }
};
}
