// Generated macro for impl_47 (impl)
macro_rules! Depcrate_punycodeimpl_47 {
() => {
// Module: crate::punycode
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : PunycodeCodeUnit + Copy , C : PunycodeCaller > ExactSizeIterator for Decode < '_ , T , C > { fn len (& self) -> usize { self . len - self . position } }
};
}
