// Generated macro for impl_66 (impl)
macro_rules! Depcrate_bstrimpl_66 {
() => {
// Module: crate::bstr
// Provides: {"impl_66"}
// Dependencies: {}
impl Drop for BasicString { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { SysFreeString (self . 0) } } } }
};
}
