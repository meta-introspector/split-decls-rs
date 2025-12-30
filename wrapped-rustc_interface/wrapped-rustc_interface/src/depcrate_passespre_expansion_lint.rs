// Generated macro for pre_expansion_lint (function)
macro_rules! Depcrate_passespre_expansion_lint {
() => {
// Module: crate::passes
// Provides: {"pre_expansion_lint"}
// Dependencies: {}
fn pre_expansion_lint < 'a > (sess : & Session , features : & Features , lint_store : & LintStore , registered_tools : & RegisteredTools , check_node : impl EarlyCheckNode < 'a > , node_name : Symbol ,) { sess . prof . generic_activity_with_arg ("pre_AST_expansion_lint_checks" , node_name . as_str ()) . run (| | { rustc_lint :: check_ast_node (sess , None , features , true , lint_store , registered_tools , None , rustc_lint :: BuiltinCombinedPreExpansionLintPass :: new () , check_node ,) ; } ,) ; }
};
}
