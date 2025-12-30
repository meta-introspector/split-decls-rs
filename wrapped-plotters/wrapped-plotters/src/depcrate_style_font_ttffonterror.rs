// Generated macro for FontError (enum)
macro_rules! Depcrate_style_font_ttfFontError {
() => {
// Module: crate::style::font::ttf
// Provides: {"FontError"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum FontError { LockError , NoSuchFont (String , String) , FontLoadError (Arc < FontLoadingError >) , GlyphError (Arc < GlyphLoadingError >) , }
};
}
