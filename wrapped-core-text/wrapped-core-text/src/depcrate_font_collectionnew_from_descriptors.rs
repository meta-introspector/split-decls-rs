// Generated macro for new_from_descriptors (function)
macro_rules! Depcrate_font_collectionnew_from_descriptors {
() => {
// Module: crate::font_collection
// Provides: {"new_from_descriptors"}
// Dependencies: {}
pub fn new_from_descriptors (descs : & CFArray < CTFontDescriptor >) -> CTFontCollection { unsafe { let key = CFString :: wrap_under_get_rule (kCTFontCollectionRemoveDuplicatesOption) ; let value = CFNumber :: from (1i64) ; let options = CFDictionary :: from_CFType_pairs (& [(key . as_CFType () , value . as_CFType ())]) ; let font_collection_ref = CTFontCollectionCreateWithFontDescriptors (descs . as_concrete_TypeRef () , options . as_concrete_TypeRef () ,) ; CTFontCollection :: wrap_under_create_rule (font_collection_ref) } }
};
}
