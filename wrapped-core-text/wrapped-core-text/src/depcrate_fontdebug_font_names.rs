// Generated macro for debug_font_names (function)
macro_rules! Depcrate_fontdebug_font_names {
() => {
// Module: crate::font
// Provides: {"debug_font_names"}
// Dependencies: {}
pub fn debug_font_names (font : & CTFont) { fn get_key (font : & CTFont , key : CTFontNameSpecifier) -> String { font . get_string_by_name_key (key) . unwrap () } println ! ("kCTFontFamilyNameKey: {}" , get_key (font , CTFontNameSpecifier :: Family)) ; println ! ("kCTFontSubFamilyNameKey: {}" , get_key (font , CTFontNameSpecifier :: SubFamily)) ; println ! ("kCTFontStyleNameKey: {}" , get_key (font , CTFontNameSpecifier :: Style)) ; println ! ("kCTFontUniqueNameKey: {}" , get_key (font , CTFontNameSpecifier :: Unique)) ; println ! ("kCTFontFullNameKey: {}" , get_key (font , CTFontNameSpecifier :: Full)) ; println ! ("kCTFontPostScriptNameKey: {}" , get_key (font , CTFontNameSpecifier :: PostScript)) ; }
};
}
