// Generated macro for lint_unnested_or_patterns (function)
macro_rules! Depcrate_unnested_or_patternslint_unnested_or_patterns {
() => {
// Module: crate::unnested_or_patterns
// Provides: {"lint_unnested_or_patterns"}
// Dependencies: {}
fn lint_unnested_or_patterns (cx : & EarlyContext < '_ > , pat : & Pat) { if let Ident (.. , None) | Expr (_) | Wild | Path (..) | Range (..) | Rest | MacCall (_) = pat . kind { return ; } let mut pat = pat . clone () ; remove_all_parens (& mut pat) ; if ! unnest_or_patterns (& mut pat) { return ; } span_lint_and_then (cx , UNNESTED_OR_PATTERNS , pat . span , "unnested or-patterns" , | db | { insert_necessary_parens (& mut pat) ; db . span_suggestion_verbose (pat . span , "nest the patterns" , pprust :: pat_to_string (& pat) , Applicability :: MachineApplicable ,) ; }) ; }
};
}
