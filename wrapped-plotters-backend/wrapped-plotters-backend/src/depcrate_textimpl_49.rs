// Generated macro for impl_49 (impl)
macro_rules! Depcrate_textimpl_49 {
() => {
// Module: crate::text
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > FontFamily < 'a > { # [doc = " Make a CSS compatible string for the font family name."] # [doc = " This can be used as the value of `font-family` attribute in SVG."] pub fn as_str (& self) -> & str { match self { FontFamily :: Serif => "serif" , FontFamily :: SansSerif => "sans-serif" , FontFamily :: Monospace => "monospace" , FontFamily :: Name (face) => face , } } }
};
}
