// Generated macro for escape_html_scalar (function)
macro_rules! Depcrateescape_html_scalar {
() => {
// Module: crate
// Provides: {"escape_html_scalar"}
// Dependencies: {}
fn escape_html_scalar < W : StrWrite > (mut w : W , s : & str , table : & 'static [u8 ; 256] ,) -> Result < () , W :: Error > { let bytes = s . as_bytes () ; let mut mark = 0 ; let mut i = 0 ; while i < s . len () { match bytes [i ..] . iter () . position (| & c | table [c as usize] != 0) { Some (pos) => { i += pos ; } None => break , } let c = bytes [i] ; let escape = table [c as usize] ; let escape_seq = HTML_ESCAPES [escape as usize] ; w . write_str (& s [mark .. i]) ? ; w . write_str (escape_seq) ? ; i += 1 ; mark = i ; } w . write_str (& s [mark ..]) }
};
}
