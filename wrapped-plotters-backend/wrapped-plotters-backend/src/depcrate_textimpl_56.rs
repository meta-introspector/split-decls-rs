// Generated macro for impl_56 (impl)
macro_rules! Depcrate_textimpl_56 {
() => {
// Module: crate::text
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a > From < & 'a str > for FontStyle { fn from (from : & 'a str) -> FontStyle { match from . to_lowercase () . as_str () { "normal" => FontStyle :: Normal , "italic" => FontStyle :: Italic , "oblique" => FontStyle :: Oblique , "bold" => FontStyle :: Bold , _ => FontStyle :: Normal , } } }
};
}
