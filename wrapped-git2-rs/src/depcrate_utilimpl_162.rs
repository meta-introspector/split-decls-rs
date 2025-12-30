// Generated macro for impl_162 (impl)
macro_rules! Depcrate_utilimpl_162 {
() => {
// Module: crate::util
// Provides: {"impl_162"}
// Dependencies: {}
impl IntoCString for Vec < u8 > { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self) ?) } }
};
}
