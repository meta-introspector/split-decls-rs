// Generated macro for display_macl (function)
macro_rules! Depcrate_fs_feature_xattrdisplay_macl {
() => {
// Module: crate::fs::feature::xattr
// Provides: {"display_macl"}
// Dependencies: {}
# [cfg (target_os = "macos")] fn display_macl (attribute : & Attribute) -> Option < String > { attribute . value . as_ref () . filter (| v | v . len () % 18 == 0) . map (| v | { let macls = v . as_slice () . chunks (18) . filter (| c | c [0] != 0 || c [1] != 0) . map (format_macl) . collect :: < Vec < String > > () . join (", ") ; format ! ("[{macls}]") }) }
};
}
