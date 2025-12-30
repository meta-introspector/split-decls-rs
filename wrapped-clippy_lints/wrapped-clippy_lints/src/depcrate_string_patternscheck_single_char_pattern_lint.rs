// Generated macro for check_single_char_pattern_lint (function)
macro_rules! Depcrate_string_patternscheck_single_char_pattern_lint {
() => {
// Module: crate::string_patterns
// Provides: {"check_single_char_pattern_lint"}
// Dependencies: {}
fn check_single_char_pattern_lint (cx : & LateContext < '_ > , arg : & Expr < '_ >) { let mut applicability = Applicability :: MachineApplicable ; if let Some (hint) = str_literal_to_char_literal (cx , arg , & mut applicability , true) { span_lint_and_sugg (cx , SINGLE_CHAR_PATTERN , arg . span , "single-character string constant used as pattern" , "consider using a `char`" , hint , applicability ,) ; } }
};
}
