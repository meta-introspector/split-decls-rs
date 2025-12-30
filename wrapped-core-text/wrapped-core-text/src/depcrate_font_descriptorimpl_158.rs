// Generated macro for impl_158 (impl)
macro_rules! Depcrate_font_descriptorimpl_158 {
() => {
// Module: crate::font_descriptor
// Provides: {"impl_158"}
// Dependencies: {}
impl StylisticClassAccessors for CTFontStylisticClass { fn is_serif (& self) -> bool { let any_serif_class = kCTFontOldStyleSerifsClass | kCTFontTransitionalSerifsClass | kCTFontModernSerifsClass | kCTFontClarendonSerifsClass | kCTFontSlabSerifsClass | kCTFontFreeformSerifsClass ; (* self & any_serif_class) != 0 } fn is_sans_serif (& self) -> bool { (* self & kCTFontSansSerifClass) != 0 } fn is_script (& self) -> bool { (* self & kCTFontScriptsClass) != 0 } fn is_fantasy (& self) -> bool { (* self & kCTFontOrnamentalsClass) != 0 } fn is_symbols (& self) -> bool { (* self & kCTFontSymbolicClass) != 0 } }
};
}
