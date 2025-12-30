// Generated macro for contains_acronym (function)
macro_rules! Depcrate_upper_case_acronymscontains_acronym {
() => {
// Module: crate::upper_case_acronyms
// Provides: {"contains_acronym"}
// Dependencies: {}
fn contains_acronym (s : & str) -> bool { let mut count = 0 ; for c in s . chars () { if c . is_ascii_uppercase () { count += 1 ; if count == 3 { return true ; } } else { count = 0 ; } } count == 2 }
};
}
