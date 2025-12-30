// Generated macro for impl_179 (impl)
macro_rules! Depcrate_builder_os_strimpl_179 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_179"}
// Dependencies: {}
impl OsStr { # [cfg (feature = "string")] pub (crate) fn from_string (name : std :: ffi :: OsString) -> Self { Self { name : Inner :: from_string (name) , } } # [cfg (feature = "string")] pub (crate) fn from_ref (name : & std :: ffi :: OsStr) -> Self { Self { name : Inner :: from_ref (name) , } } pub (crate) fn from_static_ref (name : & 'static std :: ffi :: OsStr) -> Self { Self { name : Inner :: from_static_ref (name) , } } # [doc = " Get the raw string as an `std::ffi::OsStr`"] pub fn as_os_str (& self) -> & std :: ffi :: OsStr { self . name . as_os_str () } # [doc = " Get the raw string as an `OsString`"] pub fn to_os_string (& self) -> std :: ffi :: OsString { self . as_os_str () . to_owned () } }
};
}
