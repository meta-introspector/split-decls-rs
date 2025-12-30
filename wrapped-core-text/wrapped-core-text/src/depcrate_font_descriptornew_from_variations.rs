// Generated macro for new_from_variations (function)
macro_rules! Depcrate_font_descriptornew_from_variations {
() => {
// Module: crate::font_descriptor
// Provides: {"new_from_variations"}
// Dependencies: {}
pub fn new_from_variations (variations : & CFDictionary < CFString , CFNumber >) -> CTFontDescriptor { unsafe { let var_key = CFString :: wrap_under_get_rule (kCTFontVariationAttribute) ; let var_val = CFType :: wrap_under_get_rule (variations . as_CFTypeRef ()) ; let attributes = CFDictionary :: from_CFType_pairs (& [(var_key , var_val)]) ; new_from_attributes (& attributes) } }
};
}
