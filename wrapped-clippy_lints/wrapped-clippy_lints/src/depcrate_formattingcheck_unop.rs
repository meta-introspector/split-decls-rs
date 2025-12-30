// Generated macro for check_unop (function)
macro_rules! Depcrate_formattingcheck_unop {
() => {
// Module: crate::formatting
// Provides: {"check_unop"}
// Dependencies: {}
# [doc = " Implementation of the `SUSPICIOUS_UNARY_OP_FORMATTING` lint."] fn check_unop (cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: Binary (ref binop , ref lhs , ref rhs) = expr . kind && ! lhs . span . from_expansion () && ! rhs . span . from_expansion () && let binop_span = lhs . span . between (rhs . span) && let ExprKind :: Unary (op , ref un_rhs) = rhs . kind && let unop_operand_span = rhs . span . until (un_rhs . span) && let Some (binop_snippet) = snippet_opt (cx , binop_span) && let Some (unop_operand_snippet) = snippet_opt (cx , unop_operand_span) && let binop_str = binop . node . as_str () && binop_snippet . ends_with (binop_str) && unop_operand_snippet . ends_with (' ') { let unop_str = op . as_str () ; let eqop_span = lhs . span . between (un_rhs . span) ; span_lint_and_help (cx , SUSPICIOUS_UNARY_OP_FORMATTING , eqop_span , format ! ("by not having a space between `{binop_str}` and `{unop_str}` it looks like \
                 `{binop_str}{unop_str}` is a single operator") , None , format ! ("put a space between `{binop_str}` and `{unop_str}` and remove the space after `{unop_str}`") ,) ; } }
};
}
