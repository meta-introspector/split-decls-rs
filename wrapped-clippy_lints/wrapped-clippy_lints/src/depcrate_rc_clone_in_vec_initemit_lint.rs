// Generated macro for emit_lint (function)
macro_rules! Depcrate_rc_clone_in_vec_initemit_lint {
() => {
// Module: crate::rc_clone_in_vec_init
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint (cx : & LateContext < '_ > , symbol : Symbol , lint_span : Span , elem : & Expr < '_ > , len : & Expr < '_ > , func_span : Span) { let symbol_name = symbol . as_str () ; span_lint_and_then (cx , RC_CLONE_IN_VEC_INIT , lint_span , "initializing a reference-counted pointer in `vec![elem; len]`" , | diag | { let len_snippet = snippet (cx , len . span , "..") ; let elem_snippet = format ! ("{}(..)" , snippet (cx , elem . span . with_hi (func_span . hi ()) , "..")) ; let indentation = " " . repeat (indent_of (cx , lint_span) . unwrap_or (0)) ; let loop_init_suggestion = loop_init_suggestion (& elem_snippet , len_snippet . as_ref () , & indentation) ; let extract_suggestion = extract_suggestion (& elem_snippet , len_snippet . as_ref () , & indentation) ; diag . note (format ! ("each element will point to the same `{symbol_name}` instance")) ; diag . span_suggestion (lint_span , format ! ("consider initializing each `{symbol_name}` element individually") , loop_init_suggestion , Applicability :: HasPlaceholders ,) ; diag . span_suggestion (lint_span , format ! ("or if this is intentional, consider extracting the `{symbol_name}` initialization to a variable") , extract_suggestion , Applicability :: HasPlaceholders ,) ; } ,) ; }
};
}
