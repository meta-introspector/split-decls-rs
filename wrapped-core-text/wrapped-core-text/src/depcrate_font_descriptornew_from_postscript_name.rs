// Generated macro for new_from_postscript_name (function)
macro_rules! Depcrate_font_descriptornew_from_postscript_name {
() => {
// Module: crate::font_descriptor
// Provides: {"new_from_postscript_name"}
// Dependencies: {}
pub fn new_from_postscript_name (name : & CFString) -> CTFontDescriptor { unsafe { let result : CTFontDescriptorRef = CTFontDescriptorCreateWithNameAndSize (name . as_concrete_TypeRef () , 0.0) ; CTFontDescriptor :: wrap_under_create_rule (result) } }
};
}
