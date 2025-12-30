// Generated macro for impl_1183 (impl)
macro_rules! Depcrate_style_font_ab_glyphimpl_1183 {
() => {
// Module: crate::style::font::ab_glyph
// Provides: {"impl_1183"}
// Dependencies: {}
impl FontMap { fn new () -> Self { Self { map : HashMap :: with_capacity (4) , } } fn insert (& mut self , style : FontStyle , font : FontRef < 'static >) -> Option < FontRef < 'static > > { self . map . insert (style . as_str () . to_string () , font) } fn get_fallback (& self , style : FontStyle) -> Option < & FontRef < 'static > > { self . map . get (style . as_str ()) . or_else (| | self . map . get (FontStyle :: Normal . as_str ())) } }
};
}
