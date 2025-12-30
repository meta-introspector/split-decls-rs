// Generated macro for escape_href (function)
macro_rules! Depcrateescape_href {
() => {
// Module: crate
// Provides: {"escape_href"}
// Dependencies: {}
# [doc = " Writes an href to the buffer, escaping href unsafe bytes."] pub fn escape_href < W > (mut w : W , s : & str) -> Result < () , W :: Error > where W : StrWrite , { let bytes = s . as_bytes () ; let mut mark = 0 ; for i in 0 .. bytes . len () { let c = bytes [i] ; if c >= 0x80 || HREF_SAFE [c as usize] == 0 { if mark < i { w . write_str (& s [mark .. i]) ? ; } match c { b'&' => { w . write_str (AMP_ESCAPE) ? ; } b'\'' => { w . write_str (SINGLE_QUOTE_ESCAPE) ? ; } _ => { let mut buf = [0u8 ; 3] ; buf [0] = b'%' ; buf [1] = HEX_CHARS [((c as usize) >> 4) & 0xF] ; buf [2] = HEX_CHARS [(c as usize) & 0xF] ; let escaped = from_utf8 (& buf) . unwrap () ; w . write_str (escaped) ? ; } } mark = i + 1 ; } } w . write_str (& s [mark ..]) }
};
}
