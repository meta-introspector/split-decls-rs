// Generated macro for ForLoopsOverFalliblesLoopSub (enum)
macro_rules! Depcrate_lintsForLoopsOverFalliblesLoopSub {
() => {
// Module: crate::lints
// Provides: {"ForLoopsOverFalliblesLoopSub"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ForLoopsOverFalliblesLoopSub < 'a > { # [suggestion (lint_remove_next , code = ".by_ref()" , applicability = "maybe-incorrect")] RemoveNext { # [primary_span] suggestion : Span , recv_snip : String , } , # [multipart_suggestion (lint_use_while_let , applicability = "maybe-incorrect")] UseWhileLet { # [suggestion_part (code = "while let {var}(")] start_span : Span , # [suggestion_part (code = ") = ")] end_span : Span , var : & 'a str , } , }
};
}
