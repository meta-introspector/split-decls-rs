// Generated macro for new_from_CGFont (function)
macro_rules! Depcrate_fontnew_from_CGFont {
() => {
// Module: crate::font
// Provides: {"new_from_CGFont"}
// Dependencies: {}
pub fn new_from_CGFont (cgfont : & CGFont , pt_size : f64) -> CTFont { unsafe { let font_ref = CTFontCreateWithGraphicsFont (cgfont . as_ptr () as * mut _ , pt_size as CGFloat , ptr :: null () , ptr :: null () ,) ; CTFont :: wrap_under_create_rule (font_ref) } }
};
}
