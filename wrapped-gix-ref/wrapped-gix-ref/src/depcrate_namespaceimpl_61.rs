// Generated macro for impl_61 (impl)
macro_rules! Depcrate_namespaceimpl_61 {
() => {
// Module: crate::namespace
// Provides: {"impl_61"}
// Dependencies: {}
impl Namespace { # [doc = " Dissolve ourselves into the interior representation"] pub fn into_bstring (self) -> BString { self . 0 } # [doc = " Return ourselves as"] pub fn as_bstr (& self) -> & BStr { self . 0 . as_ref () } # [doc = " Return ourselves as a path for use within the filesystem."] pub fn to_path (& self) -> & Path { gix_path :: from_byte_slice (& self . 0) } # [doc = " Append the given `prefix` to this namespace so it becomes usable for prefixed iteration."] # [doc = ""] # [doc = " The prefix is a relative path with slash-separated path components."] pub fn into_namespaced_prefix (mut self , prefix : & RelativePath) -> BString { self . 0 . push_str (prefix) ; gix_path :: to_unix_separators_on_windows (self . 0) . into_owned () } pub (crate) fn into_namespaced_name (mut self , name : & FullNameRef) -> FullName { self . 0 . push_str (name . as_bstr ()) ; FullName (self . 0) } }
};
}
