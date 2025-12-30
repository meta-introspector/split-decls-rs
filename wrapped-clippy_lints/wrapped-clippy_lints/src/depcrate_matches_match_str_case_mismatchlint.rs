// Generated macro for lint (function)
macro_rules! Depcrate_matches_match_str_case_mismatchlint {
() => {
// Module: crate::matches::match_str_case_mismatch
// Provides: {"lint"}
// Dependencies: {}
fn lint (cx : & LateContext < '_ > , case_method : & CaseMethod , bad_case_span : Span , bad_case_str : & str) { let (method_str , suggestion) = match case_method { CaseMethod :: LowerCase => ("to_lowercase" , bad_case_str . to_lowercase ()) , CaseMethod :: AsciiLowerCase => ("to_ascii_lowercase" , bad_case_str . to_ascii_lowercase ()) , CaseMethod :: UpperCase => ("to_uppercase" , bad_case_str . to_uppercase ()) , CaseMethod :: AsciiUppercase => ("to_ascii_uppercase" , bad_case_str . to_ascii_uppercase ()) , } ; span_lint_and_sugg (cx , MATCH_STR_CASE_MISMATCH , bad_case_span , "this `match` arm has a differing case than its expression" , format ! ("consider changing the case of this arm to respect `{method_str}`") , format ! ("\"{suggestion}\"") , Applicability :: MachineApplicable ,) ; }
};
}
