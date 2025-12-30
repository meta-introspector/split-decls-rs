// Generated macro for impl_50 (impl)
macro_rules! Depcrate_textimpl_50 {
() => {
// Module: crate::text
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > From < & 'a str > for FontFamily < 'a > { fn from (from : & 'a str) -> FontFamily < 'a > { match from . to_lowercase () . as_str () { "serif" => FontFamily :: Serif , "sans-serif" => FontFamily :: SansSerif , "monospace" => FontFamily :: Monospace , _ => FontFamily :: Name (from) , } } }
};
}
