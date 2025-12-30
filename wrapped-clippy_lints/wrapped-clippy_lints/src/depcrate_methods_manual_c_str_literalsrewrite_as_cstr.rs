// Generated macro for rewrite_as_cstr (function)
macro_rules! Depcrate_methods_manual_c_str_literalsrewrite_as_cstr {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"rewrite_as_cstr"}
// Dependencies: {}
# [doc = " Rewrites a byte string literal to a c-str literal."] # [doc = " `b\"foo\\0\"` -> `c\"foo\"`"] # [doc = ""] # [doc = " Returns `None` if it doesn't end in a NUL byte."] fn rewrite_as_cstr (cx : & LateContext < '_ > , span : Span) -> Option < String > { let mut sugg = String :: from ("c") + snippet (cx , span . source_callsite () , "..") . trim_start_matches ('b') ; if let Some (quote_pos) = sugg . rfind ('"') { if sugg . as_bytes () [quote_pos - 1] == b'\0' { sugg . remove (quote_pos - 1) ; } else if sugg [.. quote_pos] . ends_with ("\\x00") { sugg . replace_range (quote_pos - 4 .. quote_pos , "") ; } else if sugg [.. quote_pos] . ends_with ("\\0") { sugg . replace_range (quote_pos - 2 .. quote_pos , "") ; } else { return None ; } } Some (sugg) }
};
}
