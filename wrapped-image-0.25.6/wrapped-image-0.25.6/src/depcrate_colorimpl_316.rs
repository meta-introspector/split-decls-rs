// Generated macro for impl_316 (impl)
macro_rules! Depcrate_colorimpl_316 {
() => {
// Module: crate::color
// Provides: {"impl_316"}
// Dependencies: {}
impl < T : Primitive > Invert for Rgba < T > { fn invert (& mut self) { let rgba = self . 0 ; let max = T :: DEFAULT_MAX_VALUE ; * self = Rgba ([max - rgba [0] , max - rgba [1] , max - rgba [2] , rgba [3]]) ; } }
};
}
