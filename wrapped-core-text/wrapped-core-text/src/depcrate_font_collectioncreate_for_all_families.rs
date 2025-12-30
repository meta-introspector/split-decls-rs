// Generated macro for create_for_all_families (function)
macro_rules! Depcrate_font_collectioncreate_for_all_families {
() => {
// Module: crate::font_collection
// Provides: {"create_for_all_families"}
// Dependencies: {}
pub fn create_for_all_families () -> CTFontCollection { unsafe { let key = CFString :: wrap_under_get_rule (kCTFontCollectionRemoveDuplicatesOption) ; let value = CFNumber :: from (1i64) ; let options = CFDictionary :: from_CFType_pairs (& [(key . as_CFType () , value . as_CFType ())]) ; let font_collection_ref = CTFontCollectionCreateFromAvailableFonts (options . as_concrete_TypeRef ()) ; CTFontCollection :: wrap_under_create_rule (font_collection_ref) } }
};
}
