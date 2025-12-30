// Generated macro for impl_346 (impl)
macro_rules! Depcrate_re_traitimpl_346 {
() => {
// Module: crate::re_trait
// Provides: {"impl_346"}
// Dependencies: {}
impl < 't , R > FindCaptures < 't , R > where R : RegularExpression , R :: Text : 't { # [doc = " Return the text being searched."] pub fn text (& self) -> & 't R :: Text { self . 0 . text () } # [doc = " Return the underlying regex."] pub fn regex (& self) -> & R { self . 0 . regex () } }
};
}
