// Generated macro for reindent_multiline_inner (function)
macro_rules! Depcrate_sourcereindent_multiline_inner {
() => {
// Module: crate::source
// Provides: {"reindent_multiline_inner"}
// Dependencies: {}
fn reindent_multiline_inner (s : & str , ignore_first : bool , indent : Option < usize > , ch : char) -> String { let x = s . lines () . skip (usize :: from (ignore_first)) . filter_map (| l | { if l . is_empty () { None } else { Some (l . char_indices () . find (| & (_ , x) | x != ch) . unwrap_or ((l . len () , ch)) . 0) } }) . min () . unwrap_or (0) ; let indent = indent . unwrap_or (0) ; s . lines () . enumerate () . map (| (i , l) | { if (ignore_first && i == 0) || l . is_empty () { l . to_owned () } else if x > indent { l . split_at (x - indent) . 1 . to_owned () } else { " " . repeat (indent - x) + l } }) . collect :: < Vec < String > > () . join ("\n") }
};
}
