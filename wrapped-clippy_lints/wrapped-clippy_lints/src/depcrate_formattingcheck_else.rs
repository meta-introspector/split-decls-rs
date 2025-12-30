// Generated macro for check_else (function)
macro_rules! Depcrate_formattingcheck_else {
() => {
// Module: crate::formatting
// Provides: {"check_else"}
// Dependencies: {}
# [doc = " Implementation of the `SUSPICIOUS_ELSE_FORMATTING` lint for weird `else`."] fn check_else (cx : & EarlyContext < '_ > , expr : & Expr) { if let ExprKind :: If (_ , then , Some (else_)) = & expr . kind && (is_block (else_) || is_if (else_)) && ! then . span . from_expansion () && ! else_ . span . from_expansion () && ! expr . span . in_external_macro (cx . sess () . source_map ()) && expr . span . lo () . 0 != 0 && expr . span . hi () . 0 != 0 && let else_span = then . span . between (else_ . span) && let Some (else_snippet) = snippet_opt (cx , else_span) && let Some ((pre_else , post_else)) = else_snippet . split_once ("else") && ! else_snippet . contains ('/') && let Some ((_ , post_else_post_eol)) = post_else . split_once ('\n') { if is_block (else_) && let Some ((_ , pre_else_post_eol)) = pre_else . split_once ('\n') && ! pre_else_post_eol . contains ('\n') && ! post_else_post_eol . contains ('\n') { return ; } let trimmed_post_else_post_eol = post_else_post_eol . trim () ; if trimmed_post_else_post_eol . starts_with ("/*") && trimmed_post_else_post_eol . ends_with ("*/") { return ; } let else_desc = if is_if (else_) { "if" } else { "{..}" } ; span_lint_and_note (cx , SUSPICIOUS_ELSE_FORMATTING , else_span , format ! ("this is an `else {else_desc}` but the formatting might hide it") , None , format ! ("to remove this lint, remove the `else` or remove the new line between \
                 `else` and `{else_desc}`" ,) ,) ; } }
};
}
