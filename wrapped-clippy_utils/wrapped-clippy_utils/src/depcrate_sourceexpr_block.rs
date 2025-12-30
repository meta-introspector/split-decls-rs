// Generated macro for expr_block (function)
macro_rules! Depcrate_sourceexpr_block {
() => {
// Module: crate::source
// Provides: {"expr_block"}
// Dependencies: {}
# [doc = " Like `snippet_block`, but add braces if the expr is not an `ExprKind::Block` with no label."] pub fn expr_block (sess : & impl HasSession , expr : & Expr < '_ > , outer : SyntaxContext , default : & str , indent_relative_to : Option < Span > , app : & mut Applicability ,) -> String { let (code , from_macro) = snippet_block_with_context (sess , expr . span , outer , default , indent_relative_to , app) ; if ! from_macro && let ExprKind :: Block (block , None) = expr . kind && block . rules != BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) { code } else { format ! ("{{ {code} }}") } }
};
}
