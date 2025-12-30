// Generated macro for new_from_descriptor_and_options (function)
macro_rules! Depcrate_fontnew_from_descriptor_and_options {
() => {
// Module: crate::font
// Provides: {"new_from_descriptor_and_options"}
// Dependencies: {}
pub fn new_from_descriptor_and_options (desc : & CTFontDescriptor , pt_size : f64 , options : CTFontOptions ,) -> CTFont { unsafe { let font_ref = CTFontCreateWithFontDescriptorAndOptions (desc . as_concrete_TypeRef () , pt_size as CGFloat , ptr :: null () , options ,) ; CTFont :: wrap_under_create_rule (font_ref) } }
};
}
