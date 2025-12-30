// Generated macro for new_from_descriptor (function)
macro_rules! Depcrate_fontnew_from_descriptor {
() => {
// Module: crate::font
// Provides: {"new_from_descriptor"}
// Dependencies: {}
pub fn new_from_descriptor (desc : & CTFontDescriptor , pt_size : f64) -> CTFont { unsafe { let font_ref = CTFontCreateWithFontDescriptor (desc . as_concrete_TypeRef () , pt_size as CGFloat , ptr :: null () ,) ; CTFont :: wrap_under_create_rule (font_ref) } }
};
}
