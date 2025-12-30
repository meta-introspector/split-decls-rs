// Generated macro for get_case_method (function)
macro_rules! Depcrate_matches_match_str_case_mismatchget_case_method {
() => {
// Module: crate::matches::match_str_case_mismatch
// Provides: {"get_case_method"}
// Dependencies: {}
fn get_case_method (segment_ident : Symbol) -> Option < CaseMethod > { match segment_ident { sym :: to_lowercase => Some (CaseMethod :: LowerCase) , sym :: to_ascii_lowercase => Some (CaseMethod :: AsciiLowerCase) , sym :: to_uppercase => Some (CaseMethod :: UpperCase) , sym :: to_ascii_uppercase => Some (CaseMethod :: AsciiUppercase) , _ => None , } }
};
}
