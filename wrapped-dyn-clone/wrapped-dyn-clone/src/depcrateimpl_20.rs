// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > DynClone for T where T : Clone , { fn __clone_box (& self , _ : Private) -> * mut () { Box :: < T > :: into_raw (Box :: new (self . clone ())) as * mut () } }
};
}
