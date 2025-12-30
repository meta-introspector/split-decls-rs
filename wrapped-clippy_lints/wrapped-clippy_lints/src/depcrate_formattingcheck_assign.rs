// Generated macro for check_assign (function)
macro_rules! Depcrate_formattingcheck_assign {
() => {
// Module: crate::formatting
// Provides: {"check_assign"}
// Dependencies: {}
# [doc = " Implementation of the `SUSPICIOUS_ASSIGNMENT_FORMATTING` lint."] fn check_assign (cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: Assign (ref lhs , ref rhs , _) = expr . kind && ! lhs . span . from_expansion () && ! rhs . span . from_expansion () { let eq_span = lhs . span . between (rhs . span) ; if let ExprKind :: Unary (op , ref sub_rhs) = rhs . kind && let Some (eq_snippet) = snippet_opt (cx , eq_span) { let op = op . as_str () ; let eqop_span = lhs . span . between (sub_rhs . span) ; if eq_snippet . ends_with ('=') { span_lint_and_note (cx , SUSPICIOUS_ASSIGNMENT_FORMATTING , eqop_span , format ! ("this looks like you are trying to use `.. {op}= ..`, but you \
                                 really are doing `.. = ({op} ..)`") , None , format ! ("to remove this lint, use either `{op}=` or `= {op}`") ,) ; } } } }
};
}
