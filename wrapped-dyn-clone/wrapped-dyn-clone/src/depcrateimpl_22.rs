// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < T > DynClone for [T] where T : Clone , { fn __clone_box (& self , _ : Private) -> * mut () { Box :: < [T] > :: into_raw (self . iter () . cloned () . collect ()) as * mut () } }
};
}
