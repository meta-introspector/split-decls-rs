// Generated macro for impl_55 (impl)
macro_rules! Depcrate_ensureimpl_55 {
() => {
// Module: crate::ensure
// Provides: {"impl_55"}
// Dependencies: {}
impl < A , B > BothDebug for (A , B) where A : Debug , B : Debug , { fn __dispatch_ensure (self , msg : & 'static str) -> Error { render (msg , & self . 0 , & self . 1) } }
};
}
