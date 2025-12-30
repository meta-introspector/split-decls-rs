// Generated macro for impl_10072 (impl)
macro_rules! Depcrate_suspicious_xor_used_as_powimpl_10072 {
() => {
// Module: crate::suspicious_xor_used_as_pow
// Provides: {"impl_10072"}
// Dependencies: {}
impl LateLintPass < '_ > for ConfusingXorAndPow { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if ! expr . span . in_external_macro (cx . sess () . source_map ()) && let ExprKind :: Binary (op , left , right) = & expr . kind && op . node == BinOpKind :: BitXor && left . span . eq_ctxt (right . span) && let ExprKind :: Lit (lit_left) = & left . kind && let ExprKind :: Lit (lit_right) = & right . kind && matches ! (lit_right . node , LitKind :: Int (..) | LitKind :: Float (..)) && matches ! (lit_left . node , LitKind :: Int (..) | LitKind :: Float (..)) && NumericLiteral :: from_lit_kind (& snippet (cx , lit_right . span , "..") , & lit_right . node) . is_some_and (| x | x . is_decimal ()) { span_lint_and_then (cx , SUSPICIOUS_XOR_USED_AS_POW , expr . span , "`^` is not the exponentiation operator" , | diag | { diag . span_suggestion_verbose (expr . span , "did you mean to write" , format ! ("{}.pow({})" , lit_left . node , lit_right . node) , Applicability :: MaybeIncorrect ,) ; } ,) ; } } }
};
}
