// Generated macro for check (function)
macro_rules! Depcrate_operators_double_comparisoncheck {
() => {
// Module: crate::operators::double_comparison
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , op : BinOpKind , lhs : & Expr < '_ > , rhs : & Expr < '_ > , span : Span) { if let ExprKind :: Binary (lop , llhs , lrhs) = lhs . kind && let ExprKind :: Binary (rop , rlhs , rrhs) = rhs . kind && eq_expr_value (cx , llhs , rlhs) && eq_expr_value (cx , lrhs , rrhs) { let op = match (op , lop . node , rop . node) { (BinOpKind :: Or , BinOpKind :: Eq , BinOpKind :: Lt) | (BinOpKind :: Or , BinOpKind :: Lt , BinOpKind :: Eq) => { "<=" } , (BinOpKind :: Or , BinOpKind :: Eq , BinOpKind :: Gt) | (BinOpKind :: Or , BinOpKind :: Gt , BinOpKind :: Eq) => { ">=" } , (BinOpKind :: Or , BinOpKind :: Lt , BinOpKind :: Gt) | (BinOpKind :: Or , BinOpKind :: Gt , BinOpKind :: Lt) => { "!=" } , (BinOpKind :: And , BinOpKind :: Le , BinOpKind :: Ge) | (BinOpKind :: And , BinOpKind :: Ge , BinOpKind :: Le) => { "==" } , _ => return , } ; let mut applicability = Applicability :: MachineApplicable ; let lhs_str = snippet_with_applicability (cx , llhs . span , "" , & mut applicability) ; let rhs_str = snippet_with_applicability (cx , lrhs . span , "" , & mut applicability) ; let sugg = format ! ("{lhs_str} {op} {rhs_str}") ; span_lint_and_sugg (cx , DOUBLE_COMPARISONS , span , "this binary expression can be simplified" , "try" , sugg , applicability ,) ; } }
};
}
