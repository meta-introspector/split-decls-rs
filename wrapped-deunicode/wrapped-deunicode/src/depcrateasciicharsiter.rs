// Generated macro for AsciiCharsIter (struct)
macro_rules! DepcrateAsciiCharsIter {
() => {
// Module: crate
// Provides: {"AsciiCharsIter"}
// Dependencies: {}
# [doc = " Iterator that translates Unicode characters to ASCII strings."] # [doc = ""] # [doc = " See [`AsciiChars`] trait's `str.ascii_chars()` method."] # [doc = ""] # [doc = " Additionally, it implements `Display` for formatting strings without allocations."] # [doc = ""] # [cfg_attr (feature = "alloc" , doc = "```rust")] # [cfg_attr (not (feature = "alloc") , doc = "```rust,ignore")] # [doc = " use deunicode::AsciiChars;"] # [doc = " format!(\"what's up {}\", \"🐶\".ascii_chars());"] # [doc = "```"] # [derive (Clone)] pub struct AsciiCharsIter < 'a > { next_char : Option < Option < & 'static str > > , chars : Chars < 'a > , }
};
}
