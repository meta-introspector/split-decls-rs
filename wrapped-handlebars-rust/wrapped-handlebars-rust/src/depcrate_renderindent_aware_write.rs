// Generated macro for indent_aware_write (function)
macro_rules! Depcrate_renderindent_aware_write {
() => {
// Module: crate::render
// Provides: {"indent_aware_write"}
// Dependencies: {}
# [inline] pub fn indent_aware_write (v : & str , rc : & mut RenderContext < '_ , '_ > , out : & mut dyn Output ,) -> Result < () , RenderError > { if v . is_empty () { return Ok (()) ; } rc . set_content_produced (true) ; if ! v . starts_with (newline_matcher) && rc . get_indent_before_write () { if let Some (indent) = rc . get_indent_string () { out . write (indent) ? ; } } if let Some (indent) = rc . get_indent_string () { support :: str :: write_indented (v , indent , out) ? ; } else { out . write (v . as_ref ()) ? ; } let trailing_newline = v . ends_with (newline_matcher) ; rc . set_trailing_newline (trailing_newline) ; rc . set_indent_before_write (trailing_newline) ; Ok (()) }
};
}
