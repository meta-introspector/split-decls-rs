// Generated macro for impl_154 (impl)
macro_rules! Depcrate_utilimpl_154 {
() => {
// Module: crate::util
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'a > IntoCString for & 'a str { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self) ?) } }
};
}
