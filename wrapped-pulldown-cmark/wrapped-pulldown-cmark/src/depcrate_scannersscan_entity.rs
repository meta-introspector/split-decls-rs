// Generated macro for scan_entity (function)
macro_rules! Depcrate_scannersscan_entity {
() => {
// Module: crate::scanners
// Provides: {"scan_entity"}
// Dependencies: {}
pub (crate) fn scan_entity (bytes : & [u8]) -> (usize , Option < CowStr < 'static > >) { let mut end = 1 ; if bytes . get (end) == Some (& b'#') { end += 1 ; let (bytecount , codepoint) = if end < bytes . len () && bytes [end] | 0x20 == b'x' { end += 1 ; parse_hex (& bytes [end ..] , 6) } else { parse_decimal (& bytes [end ..] , 7) } ; end += bytecount ; return if bytecount == 0 || bytes . get (end) != Some (& b';') { (0 , None) } else { (end + 1 , Some (char_from_codepoint (codepoint) . unwrap_or ('\u{FFFD}') . into ()) ,) } ; } end += scan_while (& bytes [end ..] , is_ascii_alphanumeric) ; if bytes . get (end) == Some (& b';') { if let Some (value) = entities :: get_entity (& bytes [1 .. end]) { return (end + 1 , Some (value . into ())) ; } } (0 , None) }
};
}
