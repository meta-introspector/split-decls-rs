// Generated macro for emit_warning (function)
macro_rules! Depcrate_needless_continueemit_warning {
() => {
// Module: crate::needless_continue
// Provides: {"emit_warning"}
// Dependencies: {}
fn emit_warning (cx : & LateContext < '_ > , data : & LintData < '_ > , header : & str , typ : LintType) { let (snip , message , expr) = match typ { LintType :: ContinueInsideElseBlock => (suggestion_snippet_for_continue_inside_else (cx , data) , MSG_REDUNDANT_ELSE_BLOCK , data . else_expr ,) , LintType :: ContinueInsideThenBlock => (suggestion_snippet_for_continue_inside_if (cx , data) , MSG_ELSE_BLOCK_NOT_NEEDED , data . if_expr ,) , } ; span_lint_and_help (cx , NEEDLESS_CONTINUE , expr . span , message , None , format ! ("{header}\n{snip}") ,) ; }
};
}
