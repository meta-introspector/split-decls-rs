// Generated macro for get_postscript_names (function)
macro_rules! Depcrate_font_collectionget_postscript_names {
() => {
// Module: crate::font_collection
// Provides: {"get_postscript_names"}
// Dependencies: {}
pub fn get_postscript_names () -> CFArray < CFString > { unsafe { CFArray :: wrap_under_create_rule (CTFontManagerCopyAvailablePostScriptNames ()) } }
};
}
