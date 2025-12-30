// Generated macro for impl_155 (impl)
macro_rules! Depcrate_utilimpl_155 {
() => {
// Module: crate::util
// Provides: {"impl_155"}
// Dependencies: {}
impl IntoCString for String { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self . into_bytes ()) ?) } }
};
}
