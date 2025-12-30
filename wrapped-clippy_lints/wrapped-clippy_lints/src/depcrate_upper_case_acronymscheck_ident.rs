// Generated macro for check_ident (function)
macro_rules! Depcrate_upper_case_acronymscheck_ident {
() => {
// Module: crate::upper_case_acronyms
// Provides: {"check_ident"}
// Dependencies: {}
fn check_ident (cx : & LateContext < '_ > , ident : & Ident , hir_id : HirId , be_aggressive : bool) { let s = ident . as_str () ; let replacement = if s . len () > 2 && s . bytes () . all (| c | c . is_ascii_uppercase ()) { let mut r = String :: with_capacity (s . len ()) ; let mut s = s . chars () ; r . push (s . next () . unwrap ()) ; r . extend (s . map (| c | c . to_ascii_lowercase ())) ; r } else if be_aggressive && let unprefixed = s . trim_start_matches ('_') && unprefixed . starts_with (| c : char | c . is_ascii_uppercase ()) && contains_acronym (unprefixed) { let mut r = String :: with_capacity (s . len ()) ; let mut s = s . chars () ; let mut prev_upper = false ; while let Some (c) = s . next () { r . push (if replace (& mut prev_upper , c . is_ascii_uppercase ()) && s . clone () . next () . is_none_or (| c | c . is_ascii_uppercase ()) { c . to_ascii_lowercase () } else { c } ,) ; } r } else { return ; } ; span_lint_hir_and_then (cx , UPPER_CASE_ACRONYMS , hir_id , ident . span , format ! ("name `{ident}` contains a capitalized acronym") , | diag | { diag . span_suggestion (ident . span , "consider making the acronym lowercase, except the initial letter" , replacement , Applicability :: MaybeIncorrect ,) ; } ,) ; }
};
}
