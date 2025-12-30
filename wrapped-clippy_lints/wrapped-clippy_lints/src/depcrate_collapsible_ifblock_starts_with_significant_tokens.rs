// Generated macro for block_starts_with_significant_tokens (function)
macro_rules! Depcrate_collapsible_ifblock_starts_with_significant_tokens {
() => {
// Module: crate::collapsible_if
// Provides: {"block_starts_with_significant_tokens"}
// Dependencies: {}
fn block_starts_with_significant_tokens (cx : & LateContext < '_ > , block : & Block < '_ > , stop_at : & Expr < '_ > , lint_commented_code : bool ,) -> bool { let span = block . span . split_at (1) . 1 . until (stop_at . span) ; span_contains_non_whitespace (cx , span , lint_commented_code) }
};
}
