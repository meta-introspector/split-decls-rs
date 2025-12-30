// Generated macro for impl_76 (impl)
macro_rules! Depcrate_slice_revimpl_76 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_76"}
// Dependencies: {}
impl < T > Hash for RevSlice < T > where T : Hash , { fn hash < H : Hasher > (& self , h : & mut H) { self . len () . hash (h) ; for elt in self { elt . hash (h) } } }
};
}
