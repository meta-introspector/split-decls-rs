// Generated macro for impl_153 (impl)
macro_rules! Depcrate_utilimpl_153 {
() => {
// Module: crate::util
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a , T : IntoCString + Clone > IntoCString for & 'a T { fn into_c_string (self) -> Result < CString , Error > { self . clone () . into_c_string () } }
};
}
