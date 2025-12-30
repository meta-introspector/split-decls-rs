// Generated macro for suggest_non_zero_conversion (function)
macro_rules! Depcrate_non_zero_suggestionssuggest_non_zero_conversion {
() => {
// Module: crate::non_zero_suggestions
// Provides: {"suggest_non_zero_conversion"}
// Dependencies: {}
fn suggest_non_zero_conversion (cx : & LateContext < '_ > , expr : & Expr < '_ > , fn_name : rustc_span :: Symbol , target_non_zero_type : & str , arg_snippet : & str , applicability : Applicability ,) { let suggestion = format ! ("{target_non_zero_type}::{fn_name}({arg_snippet})") ; span_lint_and_sugg (cx , NON_ZERO_SUGGESTIONS , expr . span , format ! ("consider using `{target_non_zero_type}::{fn_name}()` for more efficient and type-safe conversion") , "replace with" , suggestion , applicability ,) ; }
};
}
