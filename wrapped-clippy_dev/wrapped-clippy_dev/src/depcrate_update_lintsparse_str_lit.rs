// Generated macro for parse_str_lit (function)
macro_rules! Depcrate_update_lintsparse_str_lit {
() => {
// Module: crate::update_lints
// Provides: {"parse_str_lit"}
// Dependencies: {}
# [doc = " Removes the line splices and surrounding quotes from a string literal"] fn parse_str_lit (s : & str) -> String { let s = s . strip_prefix ("r") . unwrap_or (s) . trim_matches ('#') ; let s = s . strip_prefix ('"') . and_then (| s | s . strip_suffix ('"')) . unwrap_or_else (| | panic ! ("expected quoted string, found `{s}`")) ; let mut res = String :: with_capacity (s . len ()) ; rustc_literal_escaper :: unescape_str (s , & mut | _ , ch | { if let Ok (ch) = ch { res . push (ch) ; } }) ; res }
};
}
