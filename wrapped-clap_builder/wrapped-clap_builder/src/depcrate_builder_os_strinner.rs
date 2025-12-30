// Generated macro for inner (module)
macro_rules! Depcrate_builder_os_strinner {
() => {
// Module: crate::builder::os_str
// Provides: {"inner"}
// Dependencies: {}
# [cfg (not (feature = "string"))] pub (crate) mod inner { # [derive (Clone)] pub (crate) struct Inner (& 'static std :: ffi :: OsStr) ; impl Inner { pub (crate) fn from_static_ref (name : & 'static std :: ffi :: OsStr) -> Self { Self (name) } pub (crate) fn as_os_str (& self) -> & std :: ffi :: OsStr { self . 0 } pub (crate) fn into_os_string (self) -> std :: ffi :: OsString { self . as_os_str () . to_owned () } } }
};
}
