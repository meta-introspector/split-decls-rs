// Generated macro for impl_49 (impl)
macro_rules! Depcrate_hyperlinkimpl_49 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_49"}
// Dependencies: {}
impl HyperlinkFormat { # [doc = " Creates an empty hyperlink format."] pub fn empty () -> HyperlinkFormat { HyperlinkFormat :: default () } # [doc = " Returns true if this format is empty."] pub fn is_empty (& self) -> bool { self . parts . is_empty () } # [doc = " Creates a [`HyperlinkConfig`] from this format and the environment"] # [doc = " given."] pub fn into_config (self , env : HyperlinkEnvironment) -> HyperlinkConfig { HyperlinkConfig :: new (env , self) } # [doc = " Returns true if the format can produce line-dependent hyperlinks."] pub (crate) fn is_line_dependent (& self) -> bool { self . is_line_dependent } }
};
}
