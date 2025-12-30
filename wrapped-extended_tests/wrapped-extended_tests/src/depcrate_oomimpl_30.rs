// Generated macro for impl_30 (impl)
macro_rules! Depcrate_oomimpl_30 {
() => {
// Module: crate::oom
// Provides: {"impl_30"}
// Dependencies: {}
impl Clone for R { fn clone (& self) -> Self { let boxed = Box :: new ((self . 0 . 0 , self . 0 . 1)) ; self . 0 . 0 . fetch_add (1 , AcqRel) ; Self (boxed) } }
};
}
