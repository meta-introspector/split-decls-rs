// Generated macro for impl_343 (impl)
macro_rules! Depcrate_re_traitimpl_343 {
() => {
// Module: crate::re_trait
// Provides: {"impl_343"}
// Dependencies: {}
impl < 't , R > FindMatches < 't , R > where R : RegularExpression , R :: Text : 't { # [doc = " Return the text being searched."] pub fn text (& self) -> & 't R :: Text { self . text } # [doc = " Return the underlying regex."] pub fn regex (& self) -> & R { & self . re } }
};
}
