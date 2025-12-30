// Generated macro for debug_font_traits (function)
macro_rules! Depcrate_fontdebug_font_traits {
() => {
// Module: crate::font
// Provides: {"debug_font_traits"}
// Dependencies: {}
pub fn debug_font_traits (font : & CTFont) { let sym = font . symbolic_traits () ; println ! ("kCTFontItalicTrait: {}" , sym . is_italic ()) ; println ! ("kCTFontBoldTrait: {}" , sym . is_bold ()) ; println ! ("kCTFontExpandedTrait: {}" , sym . is_expanded ()) ; println ! ("kCTFontCondensedTrait: {}" , sym . is_condensed ()) ; println ! ("kCTFontMonoSpaceTrait: {}" , sym . is_monospace ()) ; let traits = font . all_traits () ; println ! ("kCTFontWeightTrait: {}" , traits . normalized_weight ()) ; println ! ("kCTFontWidthTrait: {}" , traits . normalized_width ()) ; }
};
}
