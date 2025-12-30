// Generated macro for check_array (function)
macro_rules! Depcrate_formattingcheck_array {
() => {
// Module: crate::formatting
// Provides: {"check_array"}
// Dependencies: {}
# [doc = " Implementation of the `POSSIBLE_MISSING_COMMA` lint for array"] fn check_array (cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: Array (ref array) = expr . kind { for element in array { if let ExprKind :: Binary (ref op , ref lhs , _) = element . kind && has_unary_equivalent (op . node) && lhs . span . eq_ctxt (op . span) && let space_span = lhs . span . between (op . span) && let Some (space_snippet) = snippet_opt (cx , space_span) && let lint_span = lhs . span . with_lo (lhs . span . hi ()) && space_snippet . contains ('\n') && indentation (cx , op . span) <= indentation (cx , lhs . span) { span_lint_and_note (cx , POSSIBLE_MISSING_COMMA , lint_span , "possibly missing a comma here" , None , "to remove this lint, add a comma or write the expr in a single line" ,) ; } } } }
};
}
