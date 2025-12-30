// Generated macro for to_snake_case (function)
macro_rules! Depcrate_common_renameto_snake_case {
() => {
// Module: crate::common::rename
// Provides: {"to_snake_case"}
// Dependencies: {}
fn to_snake_case (s : & str , upper : bool) -> String { let mut last_lower = false ; let mut out = String :: new () ; for c in s . chars () { if c == '_' { last_lower = false ; } else if c . is_lowercase () { last_lower = true ; } else if c . is_uppercase () { if last_lower { out . push ('_') ; } last_lower = false ; } if upper { for u in c . to_uppercase () { out . push (u) ; } } else { for u in c . to_lowercase () { out . push (u) ; } } } out }
};
}
