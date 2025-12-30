// Generated macro for impl_315 (impl)
macro_rules! Depcrate_colorimpl_315 {
() => {
// Module: crate::color
// Provides: {"impl_315"}
// Dependencies: {}
impl < T : Primitive > Invert for Luma < T > { fn invert (& mut self) { let l = self . 0 ; let max = T :: DEFAULT_MAX_VALUE ; let l1 = max - l [0] ; * self = Luma ([l1]) ; } }
};
}
