// Generated macro for impl_159 (impl)
macro_rules! Depcrate_utilimpl_159 {
() => {
// Module: crate::util
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a > IntoCString for & 'a OsStr { fn into_c_string (self) -> Result < CString , Error > { self . to_os_string () . into_c_string () } }
};
}
