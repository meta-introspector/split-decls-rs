// Generated macro for custom_value_display (function)
macro_rules! Depcrate_fs_feature_xattrcustom_value_display {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"custom_value_display"}
// Dependencies: {}
fn custom_value_display (value : & [u8]) -> Option < String > { if value . starts_with (b"bplist") { plist_value_display (value) } else { None } }
};
}
