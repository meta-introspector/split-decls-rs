// Generated macro for escape_str (function)
macro_rules! Depcrateescape_str {
() => {
// Module: crate
// Provides: {"escape_str"}
// Dependencies: {}
fn escape_str (s : & str , policy : EscapePolicy) -> String { let mut escaped : String = String :: with_capacity (s . len ()) ; for c in s . chars () { if ! policy . should_escape (c) { escaped . push (c) ; continue ; } match c { '\\' => escaped . push_str ("\\\\") , '\0' => escaped . push_str ("\\0") , '\x01' ..= '\x06' | '\x0e' ..= '\x1f' | '\x7f' ..= '\u{00ff}' => { escaped . push_str (& format ! ("\\x{:04x}" , c as isize) [..]) } '\x07' => escaped . push_str ("\\a") , '\x08' => escaped . push_str ("\\b") , '\x0c' => escaped . push_str ("\\f") , '\x0b' => escaped . push_str ("\\v") , '\n' => escaped . push_str ("\\n") , '\t' => escaped . push_str ("\\t") , '\r' => escaped . push_str ("\\r") , '\u{0080}' ..= '\u{FFFF}' => escaped . push_str (& format ! ("\\x{:04x}" , c as isize) [..]) , '\u{10000}' ..= '\u{FFFFF}' => escaped . push_str (& format ! ("\\x{:05x}" , c as isize) [..]) , '\u{100000}' ..= '\u{10FFFF}' => escaped . push_str (& format ! ("\\x{:06x}" , c as isize) [..]) , _ => { escaped . push ('\\') ; escaped . push (c) ; } } } escaped }
};
}
