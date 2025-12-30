// Generated macro for escape_html_body_text (function)
macro_rules! Depcrateescape_html_body_text {
() => {
// Module: crate
// Provides: {"escape_html_body_text"}
// Dependencies: {}
# [doc = " For use in HTML body text, writes the given string to the Write sink,"] # [doc = " replacing special HTML bytes (<, >, &) by escape sequences."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " This function should be used for escaping text nodes, not attributes."] # [doc = " In the below example, the word \"foo\" is an attribute, and the word"] # [doc = " \"bar\" is an text node. The word \"bar\" could be escaped by this function,"] # [doc = " but the word \"foo\" must be escaped using [`escape_html`]."] # [doc = ""] # [doc = " ```html"] # [doc = " <span class=\"foo\">bar</span>"] # [doc = " ```"] # [doc = ""] # [doc = " If you aren't sure what the difference is, use [`escape_html`]."] # [doc = " It should always be correct, but will produce larger output."] # [doc = ""] # [doc = " </div>"] pub fn escape_html_body_text < W : StrWrite > (w : W , s : & str) -> Result < () , W :: Error > { # [cfg (all (target_arch = "x86_64" , feature = "simd"))] { simd :: escape_html (w , s , & HTML_BODY_TEXT_ESCAPE_TABLE) } # [cfg (not (all (target_arch = "x86_64" , feature = "simd")))] { escape_html_scalar (w , s , & HTML_BODY_TEXT_ESCAPE_TABLE) } }
};
}
