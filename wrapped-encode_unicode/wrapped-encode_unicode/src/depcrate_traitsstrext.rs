// Generated macro for StrExt (trait)
macro_rules! Depcrate_traitsStrExt {
() => {
// Module: crate::traits
// Provides: {"StrExt"}
// Dependencies: {}
# [doc = " Adds `.utf8chars()` and `.utf16chars()` iterator constructors to `&str`."] pub trait StrExt : AsRef < str > { # [doc = " Equivalent to `.chars()` but produces `Utf8Char`s."] fn utf8chars (& self) -> Utf8Chars ; # [doc = " Equivalent to `.chars()` but produces `Utf16Char`s."] fn utf16chars (& self) -> Utf16Chars ; # [doc = " Equivalent to `.char_indices()` but produces `Utf8Char`s."] fn utf8char_indices (& self) -> Utf8CharIndices ; # [doc = " Equivalent to `.char_indices()` but produces `Utf16Char`s."] fn utf16char_indices (& self) -> Utf16CharIndices ; }
};
}
