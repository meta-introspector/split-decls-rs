// Generated macro for impl_207 (impl)
macro_rules! Depcrate_builder_os_strimpl_207 {
() => {
// Module: crate::builder::os_str
// Provides: {"impl_207"}
// Dependencies: {}
impl PartialEq < OsStr > for String { # [inline] fn eq (& self , other : & OsStr) -> bool { PartialEq :: eq (self . as_str () , other . as_os_str ()) } }
};
}
