// Generated macro for unicode_property_names (function)
macro_rules! Depcrate_unicodeunicode_property_names {
() => {
// Module: crate::unicode
// Provides: {"unicode_property_names"}
// Dependencies: {}
# [doc = " Return all available unicode property names"] pub fn unicode_property_names () -> Box < dyn Iterator < Item = & 'static str > > { Box :: new (BINARY_PROPERTY_NAMES . iter () . map (| name | * name) . chain (CATEGORY_PROPERTY_NAMES . iter () . map (| name | * name)) . chain (SCRIPT_PROPERTY_NAMES . iter () . map (| name | * name)) ,) }
};
}
