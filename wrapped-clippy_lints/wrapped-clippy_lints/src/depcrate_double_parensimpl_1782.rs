// Generated macro for impl_1782 (impl)
macro_rules! Depcrate_double_parensimpl_1782 {
() => {
// Module: crate::double_parens
// Provides: {"impl_1782"}
// Dependencies: {}
impl EarlyLintPass for DoubleParens { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { let span = match & expr . kind { ExprKind :: Paren (in_paren) if matches ! (in_paren . kind , ExprKind :: Paren (_) | ExprKind :: Tup (_)) => expr . span , ExprKind :: Call (_ , params) if let [param] = & * * params && let ExprKind :: Paren (_) = param . kind => { param . span } , ExprKind :: MethodCall (call) if let [arg] = & * call . args && let ExprKind :: Paren (_) = arg . kind => { arg . span } , _ => return , } ; if ! expr . span . from_expansion () { span_lint (cx , DOUBLE_PARENS , span , "consider removing unnecessary double parentheses" ,) ; } } }
};
}
