// Generated macro for check (function)
macro_rules! Depcrate_misc_early_zero_prefixed_literalcheck {
() => {
// Module: crate::misc_early::zero_prefixed_literal
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , lit_span : Span , lit_snip : & str) { let trimmed_lit_snip = lit_snip . trim_start_matches (['_' , '0']) ; span_lint_and_then (cx , ZERO_PREFIXED_LITERAL , lit_span , "this is a decimal constant" , | diag | { diag . span_suggestion (lit_span , "if you mean to use a decimal constant, remove the `0` to avoid confusion" , trimmed_lit_snip . to_string () , Applicability :: MaybeIncorrect ,) ; if ! lit_snip . contains (['8' , '9']) { diag . span_suggestion (lit_span , "if you mean to use an octal constant, use `0o`" , format ! ("0o{trimmed_lit_snip}") , Applicability :: MaybeIncorrect ,) ; } } ,) ; }
};
}
