// Generated macro for Options (struct)
macro_rules! Depcrate_blob_platform_mergeOptions {
() => {
// Module: crate::blob::platform::merge
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for the use in the [`PlatformRef::merge()`] call."] # [derive (Default , Copy , Clone , Debug , Eq , PartialEq)] pub struct Options { # [doc = " If `true`, the resources being merged are contained in a virtual ancestor,"] # [doc = " which is the case when merge bases are merged into one."] # [doc = " This flag affects the choice of merge drivers."] pub is_virtual_ancestor : bool , # [doc = " Determine how to resolve conflicts. If `None`, no conflict resolution is possible, and it picks a side."] pub resolve_binary_with : Option < builtin_driver :: binary :: ResolveWith > , # [doc = " Options for the builtin [text driver](crate::blob::BuiltinDriver::Text)."] pub text : builtin_driver :: text :: Options , }
};
}
