// Generated macro for new_from_attributes (function)
macro_rules! Depcrate_font_descriptornew_from_attributes {
() => {
// Module: crate::font_descriptor
// Provides: {"new_from_attributes"}
// Dependencies: {}
pub fn new_from_attributes (attributes : & CFDictionary < CFString , CFType >) -> CTFontDescriptor { unsafe { let result : CTFontDescriptorRef = CTFontDescriptorCreateWithAttributes (attributes . as_concrete_TypeRef ()) ; CTFontDescriptor :: wrap_under_create_rule (result) } }
};
}
