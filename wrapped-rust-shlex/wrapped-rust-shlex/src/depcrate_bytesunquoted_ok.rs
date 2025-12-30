// Generated macro for unquoted_ok (function)
macro_rules! Depcrate_bytesunquoted_ok {
() => {
// Module: crate::bytes
// Provides: {"unquoted_ok"}
// Dependencies: {}
# [doc = " Is this ASCII byte okay to emit unquoted?"] const fn unquoted_ok (c : u8) -> bool { match c as char { '+' | '-' | '.' | '/' | ':' | '@' | ']' | '_' | '0' ..= '9' | 'A' ..= 'Z' | 'a' ..= 'z' => true , '|' | '&' | ';' | '<' | '>' | '(' | ')' | '$' | '`' | '\\' | '"' | '\'' | ' ' | '\t' | '\n' | '*' | '?' | '[' | '#' | '~' | '=' | '%' | '{' | '}' | ',' | '\r' | '!' | '^' | '\x00' ..= '\x1f' | '\x7f' => false , '\u{80}' ..= '\u{10ffff}' => { unquoted_ok (c) } , } }
};
}
