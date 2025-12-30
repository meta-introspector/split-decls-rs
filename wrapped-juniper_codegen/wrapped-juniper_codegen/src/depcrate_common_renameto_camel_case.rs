// Generated macro for to_camel_case (function)
macro_rules! Depcrate_common_renameto_camel_case {
() => {
// Module: crate::common::rename
// Provides: {"to_camel_case"}
// Dependencies: {}
fn to_camel_case (s : & str) -> String { let mut dest = String :: new () ; let s_iter = if let Some (s) = s . strip_prefix ("__") { dest . push_str ("__") ; s } else { s . strip_prefix ('_') . unwrap_or (s) } . split ('_') . enumerate () ; for (i , part) in s_iter { if i > 0 && part . len () == 1 { dest . push_str (& part . to_uppercase ()) ; } else if i > 0 && part . len () > 1 { let first = part . chars () . next () . unwrap () . to_uppercase () . collect :: < String > () ; let second = & part [1 ..] ; dest . push_str (& first) ; dest . push_str (second) ; } else if i == 0 { dest . push_str (part) ; } } dest }
};
}
