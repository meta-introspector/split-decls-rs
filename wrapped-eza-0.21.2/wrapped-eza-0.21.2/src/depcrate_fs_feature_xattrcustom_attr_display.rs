// Generated macro for custom_attr_display (function)
macro_rules! Depcrate_fs_feature_xattrcustom_attr_display {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"custom_attr_display"}
// Dependencies: {}
fn custom_attr_display (attribute : & Attribute) -> Option < String > { let name = attribute . name . as_str () ; # [cfg (target_os = "macos")] let name = name . rsplit_once ('#') . map_or (name , | n | n . 0) ; ATTRIBUTE_DISPLAYS . iter () . find (| c | c . attribute == name) . and_then (| c | (c . display) (attribute)) }
};
}
