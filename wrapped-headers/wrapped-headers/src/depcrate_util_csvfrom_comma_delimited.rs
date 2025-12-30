// Generated macro for from_comma_delimited (function)
macro_rules! Depcrate_util_csvfrom_comma_delimited {
() => {
// Module: crate::util::csv
// Provides: {"from_comma_delimited"}
// Dependencies: {}
# [doc = " Reads a comma-delimited raw header into a Vec."] pub (crate) fn from_comma_delimited < 'i , I , T , E > (values : & mut I) -> Result < E , Error > where I : Iterator < Item = & 'i HeaderValue > , T : :: std :: str :: FromStr , E : :: std :: iter :: FromIterator < T > , { values . flat_map (| value | { value . to_str () . into_iter () . flat_map (| string | { let mut in_quotes = false ; string . split (move | c | { # [allow (clippy :: collapsible_else_if)] if in_quotes { if c == '"' { in_quotes = false ; } false } else { if c == ',' { true } else { if c == '"' { in_quotes = true ; } false } } }) . filter_map (| x | match x . trim () { "" => None , y => Some (y) , }) . map (| x | x . parse () . map_err (| _ | Error :: invalid ())) }) }) . collect () }
};
}
