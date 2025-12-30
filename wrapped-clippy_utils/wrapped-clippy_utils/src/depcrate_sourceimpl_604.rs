// Generated macro for impl_604 (impl)
macro_rules! Depcrate_sourceimpl_604 {
() => {
// Module: crate::source
// Provides: {"impl_604"}
// Dependencies: {}
impl SourceText { # [doc = " Takes ownership of the source file handle if the source text is accessible."] pub fn new (text : SourceFileRange) -> Option < Self > { if text . as_str () . is_some () { Some (Self (text)) } else { None } } # [doc = " Gets the source text."] pub fn as_str (& self) -> & str { self . 0 . as_str () . unwrap () } # [doc = " Converts this into an owned string."] pub fn to_owned (& self) -> String { self . as_str () . to_owned () } }
};
}
