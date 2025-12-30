// Generated macro for impl_314 (impl)
macro_rules! Depcrate_colorimpl_314 {
() => {
// Module: crate::color
// Provides: {"impl_314"}
// Dependencies: {}
impl < T : Primitive > Invert for LumaA < T > { fn invert (& mut self) { let l = self . 0 ; let max = T :: DEFAULT_MAX_VALUE ; * self = LumaA ([max - l [0] , l [1]]) ; } }
};
}
