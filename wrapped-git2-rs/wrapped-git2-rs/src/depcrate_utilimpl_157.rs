// Generated macro for impl_157 (impl)
macro_rules! Depcrate_utilimpl_157 {
() => {
// Module: crate::util
// Provides: {"impl_157"}
// Dependencies: {}
impl < 'a > IntoCString for & 'a Path { fn into_c_string (self) -> Result < CString , Error > { let s : & OsStr = self . as_ref () ; s . into_c_string () } }
};
}
