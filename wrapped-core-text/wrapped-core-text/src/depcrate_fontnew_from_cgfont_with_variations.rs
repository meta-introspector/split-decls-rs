// Generated macro for new_from_CGFont_with_variations (function)
macro_rules! Depcrate_fontnew_from_CGFont_with_variations {
() => {
// Module: crate::font
// Provides: {"new_from_CGFont_with_variations"}
// Dependencies: {}
pub fn new_from_CGFont_with_variations (cgfont : & CGFont , pt_size : f64 , variations : & CFDictionary < CFString , CFNumber > ,) -> CTFont { unsafe { let font_desc = font_descriptor :: new_from_variations (variations) ; let font_ref = CTFontCreateWithGraphicsFont (cgfont . as_ptr () as * mut _ , pt_size as CGFloat , ptr :: null () , font_desc . as_concrete_TypeRef () ,) ; CTFont :: wrap_under_create_rule (font_ref) } }
};
}
