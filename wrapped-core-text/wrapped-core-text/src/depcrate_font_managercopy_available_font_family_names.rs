// Generated macro for copy_available_font_family_names (function)
macro_rules! Depcrate_font_managercopy_available_font_family_names {
() => {
// Module: crate::font_manager
// Provides: {"copy_available_font_family_names"}
// Dependencies: {}
pub fn copy_available_font_family_names () -> CFArray < CFString > { unsafe { TCFType :: wrap_under_create_rule (CTFontManagerCopyAvailableFontFamilyNames ()) } }
};
}
