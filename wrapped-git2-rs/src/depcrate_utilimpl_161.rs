// Generated macro for impl_161 (impl)
macro_rules! Depcrate_utilimpl_161 {
() => {
// Module: crate::util
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a > IntoCString for & 'a [u8] { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self) ?) } }
};
}
