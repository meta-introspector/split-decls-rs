// Generated macro for impl_4002 (impl)
macro_rules! Depcrate_manual_assertimpl_4002 {
() => {
// Module: crate::manual_assert
// Provides: {"impl_4002"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ManualAssert { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if let Some (higher :: If { cond , then , r#else : None }) = higher :: If :: hir (expr) && ! matches ! (cond . kind , ExprKind :: Let (_)) && ! expr . span . from_expansion () && let then = peel_blocks_with_stmt (then) && let Some (macro_call) = root_macro_call (then . span) && is_panic (cx , macro_call . def_id) && ! cx . tcx . sess . source_map () . is_multiline (cond . span) && let Ok (panic_snippet) = cx . sess () . source_map () . span_to_snippet (macro_call . span) && let Some (panic_snippet) = panic_snippet . strip_suffix (')') && let Some ((_ , format_args_snip)) = panic_snippet . split_once ('(') && ! is_else_clause (cx . tcx , expr) { let mut applicability = Applicability :: MachineApplicable ; let mut comments = span_extract_comment (cx . sess () . source_map () , expr . span) ; if ! comments . is_empty () { comments += "\n" ; } let cond_sugg = ! sugg :: Sugg :: hir_with_context (cx , cond , expr . span . ctxt () , ".." , & mut applicability) ; let semicolon = if is_parent_stmt (cx , expr . hir_id) { ";" } else { "" } ; let sugg = format ! ("assert!({cond_sugg}, {format_args_snip}){semicolon}") ; span_lint_and_then (cx , MANUAL_ASSERT , expr . span , "only a `panic!` in `if`-then statement" , | diag | { if ! comments . is_empty () { diag . tool_only_span_suggestion (expr . span . shrink_to_lo () , "add comments back" , comments , applicability ,) ; } diag . span_suggestion (expr . span , "try instead" , sugg , applicability) ; } ,) ; } } }
};
}
