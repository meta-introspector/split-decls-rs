// Generated macro for check_has_no_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blockscheck_has_no_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"check_has_no_safety_comment"}
// Dependencies: {}
fn check_has_no_safety_comment (cx : & LateContext < '_ > , item : & hir :: Item < '_ >) { if let ItemKind :: Impl (Impl { of_trait : Some (of_trait) , .. }) = item . kind && of_trait . safety . is_unsafe () && ! is_lint_allowed (cx , UNDOCUMENTED_UNSAFE_BLOCKS , item . hir_id ()) && ! is_unsafe_from_proc_macro (cx , item . span) { let source_map = cx . tcx . sess . source_map () ; let span = if source_map . is_multiline (item . span) { source_map . span_until_char (item . span , '\n') } else { item . span } ; # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , UNDOCUMENTED_UNSAFE_BLOCKS , span , "unsafe impl missing a safety comment" , | diag | { diag . help ("consider adding a safety comment on the preceding line") ; } ,) ; } }
};
}
