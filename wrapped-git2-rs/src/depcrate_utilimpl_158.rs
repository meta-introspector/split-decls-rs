// Generated macro for impl_158 (impl)
macro_rules! Depcrate_utilimpl_158 {
() => {
// Module: crate::util
// Provides: {"impl_158"}
// Dependencies: {}
impl IntoCString for PathBuf { fn into_c_string (self) -> Result < CString , Error > { let s : OsString = self . into () ; s . into_c_string () } }
};
}
