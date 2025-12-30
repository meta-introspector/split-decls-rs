// Generated macro for FONTS (static)
macro_rules! Depcrate_style_font_ab_glyphFONTS {
() => {
// Module: crate::style::font::ab_glyph
// Provides: {"FONTS"}
// Dependencies: {}
static FONTS : Lazy < RwLock < HashMap < String , FontMap > > > = Lazy :: new (| | RwLock :: new (HashMap :: new ())) ;
};
}
