// Generated macro for escape_html (function)
macro_rules! Depcrateescape_html {
() => {
// Module: crate
// Provides: {"escape_html"}
// Dependencies: {}
# [doc = " Writes the given string to the Write sink, replacing special HTML bytes"] # [doc = " (<, >, &, \", ') by escape sequences."] # [doc = ""] # [doc = " Use this function to write output to quoted HTML attributes."] # [doc = " Since this function doesn't escape spaces, unquoted attributes"] # [doc = " cannot be used. For example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut value = String::new();"] # [doc = " pulldown_cmark_escape::escape_html(&mut value, \"two words\")"] # [doc = "     .expect(\"writing to a string is infallible\");"] # [doc = " // This is okay."] # [doc = " let ok = format!(\"<a title='{value}'>test</a>\");"] # [doc = " // This is not okay."] # [doc = " //let not_ok = format!(\"<a title={value}>test</a>\");"] # [doc = " ````"] pub fn escape_html < W : StrWrite > (w : W , s : & str) -> Result < () , W :: Error > { # [cfg (all (target_arch = "x86_64" , feature = "simd"))] { simd :: escape_html (w , s , & HTML_ESCAPE_TABLE) } # [cfg (not (all (target_arch = "x86_64" , feature = "simd")))] { escape_html_scalar (w , s , & HTML_ESCAPE_TABLE) } }
};
}
