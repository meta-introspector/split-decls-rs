// Generated macro for create_for_family (function)
macro_rules! Depcrate_font_collectioncreate_for_family {
() => {
// Module: crate::font_collection
// Provides: {"create_for_family"}
// Dependencies: {}
pub fn create_for_family (family : & str) -> Option < CTFontCollection > { use crate :: font_descriptor :: kCTFontFamilyNameAttribute ; unsafe { let family_attr = CFString :: wrap_under_get_rule (kCTFontFamilyNameAttribute) ; let family_name : CFString = family . parse () . unwrap () ; let specified_attrs = CFDictionary :: from_CFType_pairs (& [(family_attr . clone () , family_name . as_CFType ())]) ; let wildcard_desc : CTFontDescriptor = font_descriptor :: new_from_attributes (& specified_attrs) ; let mandatory_attrs = CFSet :: from_slice (& [family_attr . as_CFType ()]) ; let matched_descs = CTFontDescriptorCreateMatchingFontDescriptors (wildcard_desc . as_concrete_TypeRef () , mandatory_attrs . as_concrete_TypeRef () ,) ; if matched_descs . is_null () { return None ; } let matched_descs = CFArray :: wrap_under_create_rule (matched_descs) ; Some (new_from_descriptors (& matched_descs)) } }
};
}
