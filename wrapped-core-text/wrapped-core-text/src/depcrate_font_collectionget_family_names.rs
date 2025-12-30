// Generated macro for get_family_names (function)
macro_rules! Depcrate_font_collectionget_family_names {
() => {
// Module: crate::font_collection
// Provides: {"get_family_names"}
// Dependencies: {}
pub fn get_family_names () -> CFArray < CFString > { unsafe { CFArray :: wrap_under_create_rule (CTFontManagerCopyAvailableFontFamilyNames ()) } }
};
}
