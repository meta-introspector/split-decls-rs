// Generated macro for to_camel_case (function)
macro_rules! Depcrate_nonstandard_styleto_camel_case {
() => {
// Module: crate::nonstandard_style
// Provides: {"to_camel_case"}
// Dependencies: {}
fn to_camel_case (s : & str) -> String { s . trim_matches ('_') . split ('_') . filter (| component | ! component . is_empty ()) . map (| component | { let mut camel_cased_component = String :: new () ; let mut new_word = true ; let mut prev_is_lower_case = true ; for c in component . chars () { if prev_is_lower_case && c . is_uppercase () { new_word = true ; } if new_word { camel_cased_component . extend (c . to_uppercase ()) ; } else { camel_cased_component . extend (c . to_lowercase ()) ; } prev_is_lower_case = c . is_lowercase () ; new_word = false ; } camel_cased_component }) . fold ((String :: new () , None) , | (acc , prev) : (String , Option < String >) , next | { let join = if let Some (prev) = prev { let l = prev . chars () . last () . unwrap () ; let f = next . chars () . next () . unwrap () ; ! char_has_case (l) && ! char_has_case (f) } else { false } ; (acc + if join { "_" } else { "" } + & next , Some (next)) }) . 0 }
};
}
