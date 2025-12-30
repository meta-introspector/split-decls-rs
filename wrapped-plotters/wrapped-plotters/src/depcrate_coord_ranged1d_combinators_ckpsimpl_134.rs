// Generated macro for impl_134 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsimpl_134 {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"impl_134"}
// Dependencies: {}
impl < R : Ranged > WithKeyPointMethod < R > { # [doc = " Define the light key point algorithm, by default this returns an empty set"] pub fn with_light_point_func < F : Fn (usize) -> Vec < R :: ValueType > + 'static > (mut self , func : F ,) -> Self { self . light_func = Box :: new (func) ; self } }
};
}
