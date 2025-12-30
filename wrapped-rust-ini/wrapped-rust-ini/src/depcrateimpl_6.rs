// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl EscapePolicy { fn escape_basics (self) -> bool { self != EscapePolicy :: Nothing } fn escape_reserved (self) -> bool { matches ! (self , EscapePolicy :: Reserved | EscapePolicy :: ReservedUnicode | EscapePolicy :: ReservedUnicodeExtended | EscapePolicy :: Everything) } fn escape_unicode (self) -> bool { matches ! (self , EscapePolicy :: BasicsUnicode | EscapePolicy :: BasicsUnicodeExtended | EscapePolicy :: ReservedUnicode | EscapePolicy :: ReservedUnicodeExtended | EscapePolicy :: Everything) } fn escape_unicode_extended (self) -> bool { matches ! (self , EscapePolicy :: BasicsUnicodeExtended | EscapePolicy :: ReservedUnicodeExtended | EscapePolicy :: Everything) } # [doc = " Given a character this returns true if it should be escaped as"] # [doc = " per this policy or false if not."] pub fn should_escape (self , c : char) -> bool { match c { '\\' | '\x00' ..= '\x1f' | '\x7f' => self . escape_basics () , ';' | '#' | '=' | ':' => self . escape_reserved () , '\u{0080}' ..= '\u{FFFF}' => self . escape_unicode () , '\u{10000}' ..= '\u{10FFFF}' => self . escape_unicode_extended () , _ => false , } } }
};
}
