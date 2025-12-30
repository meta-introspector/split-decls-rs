// Generated macro for fulfill_or_allowed (function)
macro_rules! Depcratefulfill_or_allowed {
() => {
// Module: crate
// Provides: {"fulfill_or_allowed"}
// Dependencies: {}
# [doc = " Returns `true` if the lint is `#[allow]`ed or `#[expect]`ed at any of the `ids`, fulfilling all"] # [doc = " of the expectations in `ids`"] # [doc = ""] # [doc = " This should only be used when the lint would otherwise be emitted, for a way to check if a lint"] # [doc = " is allowed early to skip work see [`is_lint_allowed`]"] # [doc = ""] # [doc = " To emit at a lint at a different context than the one current see"] # [doc = " [`span_lint_hir`](diagnostics::span_lint_hir) or"] # [doc = " [`span_lint_hir_and_then`](diagnostics::span_lint_hir_and_then)"] pub fn fulfill_or_allowed (cx : & LateContext < '_ > , lint : & 'static Lint , ids : impl IntoIterator < Item = HirId >) -> bool { let mut suppress_lint = false ; for id in ids { let LevelAndSource { level , lint_id , .. } = cx . tcx . lint_level_at_node (lint , id) ; if let Some (expectation) = lint_id { cx . fulfill_expectation (expectation) ; } match level { Level :: Allow | Level :: Expect => suppress_lint = true , Level :: Warn | Level :: ForceWarn | Level :: Deny | Level :: Forbid => { } , } } suppress_lint }
};
}
