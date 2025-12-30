// Generated macro for other_227 (other)
macro_rules! Depcrate_fontother_227 {
() => {
// Module: crate::font
// Provides: {"other_227"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGFontCreateWithDataProvider (provider : crate :: sys :: CGDataProviderRef ,) -> crate :: sys :: CGFontRef ; fn CGFontCreateWithFontName (name : CFStringRef) -> crate :: sys :: CGFontRef ; fn CGFontCreateCopyWithVariations (font : crate :: sys :: CGFontRef , vars : CFDictionaryRef ,) -> crate :: sys :: CGFontRef ; fn CGFontGetTypeID () -> CFTypeID ; fn CGFontCopyPostScriptName (font : crate :: sys :: CGFontRef) -> CFStringRef ; fn CGFontGetGlyphBBoxes (font : crate :: sys :: CGFontRef , glyphs : * const CGGlyph , count : usize , bboxes : * mut CGRect ,) -> bool ; fn CGFontGetGlyphAdvances (font : crate :: sys :: CGFontRef , glyphs : * const CGGlyph , count : usize , advances : * mut c_int ,) -> bool ; fn CGFontGetAscent (font : crate :: sys :: CGFontRef) -> c_int ; fn CGFontGetDescent (font : crate :: sys :: CGFontRef) -> c_int ; fn CGFontGetLeading (font : crate :: sys :: CGFontRef) -> c_int ; fn CGFontGetCapHeight (font : crate :: sys :: CGFontRef) -> c_int ; fn CGFontGetXHeight (font : crate :: sys :: CGFontRef) -> c_int ; fn CGFontGetUnitsPerEm (font : crate :: sys :: CGFontRef) -> c_int ; fn CGFontCopyTableTags (font : crate :: sys :: CGFontRef) -> CFArrayRef ; fn CGFontCopyTableForTag (font : crate :: sys :: CGFontRef , tag : u32) -> CFDataRef ; fn CGFontCopyVariations (font : crate :: sys :: CGFontRef) -> CFDictionaryRef ; fn CGFontCopyVariationAxes (font : crate :: sys :: CGFontRef) -> CFArrayRef ; }
};
}
