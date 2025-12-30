// Generated macro for AsciiChars (trait)
macro_rules! DepcrateAsciiChars {
() => {
// Module: crate
// Provides: {"AsciiChars"}
// Dependencies: {}
# [doc = " Convenience functions for deunicode. `use deunicode::AsciiChars`"] pub trait AsciiChars { # [doc = " Iterate over Unicode characters converted to ASCII sequences."] # [doc = ""] # [doc = " Items of this iterator may be `None` for some characters."] # [doc = " Use `.map(|ch| ch.unwrap_or(\"?\"))` to replace invalid characters."] # [doc = ""] # [doc = " Alternatively, this iterator can be used in formatters:"] # [cfg_attr (feature = "alloc" , doc = "```rust")] # [cfg_attr (not (feature = "alloc") , doc = "```rust,ignore")] # [doc = " use deunicode::AsciiChars;"] # [doc = " format!(\"what's up {}\", \"🐶\".ascii_chars());"] # [doc = "```"] fn ascii_chars (& self) -> AsciiCharsIter < '_ > ; # [doc = " Convert any Unicode string to ASCII-only string."] # [doc = ""] # [doc = " Characters are converted to closest ASCII equivalent."] # [doc = " Characters that can't be converted are replaced with `\"[?]\"`."] # [cfg (feature = "alloc")] fn to_ascii_lossy (& self) -> String ; }
};
}
