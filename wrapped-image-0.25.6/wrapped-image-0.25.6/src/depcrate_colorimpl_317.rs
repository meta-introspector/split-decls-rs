// Generated macro for impl_317 (impl)
macro_rules! Depcrate_colorimpl_317 {
() => {
// Module: crate::color
// Provides: {"impl_317"}
// Dependencies: {}
impl < T : Primitive > Invert for Rgb < T > { fn invert (& mut self) { let rgb = self . 0 ; let max = T :: DEFAULT_MAX_VALUE ; let r1 = max - rgb [0] ; let g1 = max - rgb [1] ; let b1 = max - rgb [2] ; * self = Rgb ([r1 , g1 , b1]) ; } }
};
}
