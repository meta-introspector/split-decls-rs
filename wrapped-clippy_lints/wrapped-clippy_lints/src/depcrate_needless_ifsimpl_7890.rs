// Generated macro for impl_7890 (impl)
macro_rules! Depcrate_needless_ifsimpl_7890 {
() => {
// Module: crate::needless_ifs
// Provides: {"impl_7890"}
// Dependencies: {}
impl LateLintPass < '_ > for NeedlessIfs { fn check_stmt < 'tcx > (& mut self , cx : & LateContext < 'tcx > , stmt : & Stmt < 'tcx >) { if let StmtKind :: Expr (expr) = stmt . kind && let Some (If { cond , then , r#else : None , }) = If :: hir (expr) && let ExprKind :: Block (block , ..) = then . kind && block . stmts . is_empty () && block . expr . is_none () && ! expr . span . in_external_macro (cx . sess () . source_map ()) && then . span . check_source_text (cx , | src | { src . bytes () . all (| ch | matches ! (ch , b'{' | b'}') || ch . is_ascii_whitespace ()) }) && let Some (cond_span) = walk_span_to_context (cond . span , expr . span . ctxt ()) && let Some (cond_snippet) = cond_span . get_source_text (cx) && ! is_from_proc_macro (cx , expr) { span_lint_and_sugg (cx , NEEDLESS_IFS , stmt . span , "this `if` branch is empty" , "you can remove it" , if cond . can_have_side_effects () || ! cx . tcx . hir_attrs (stmt . hir_id) . is_empty () { if cond_snippet . starts_with ('{') { format ! ("({cond_snippet});") } else { format ! ("{cond_snippet};") } } else { String :: new () } , Applicability :: MachineApplicable ,) ; } } }
};
}
