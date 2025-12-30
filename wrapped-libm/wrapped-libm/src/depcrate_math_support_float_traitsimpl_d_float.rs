// Generated macro for impl_d_float (macro)
macro_rules! Depcrate_math_support_float_traitsimpl_d_float {
() => {
// Module: crate::math::support::float_traits
// Provides: {"impl_d_float"}
// Dependencies: {}
macro_rules ! impl_d_float { ($ ($ X : ident $ D : ident) ,*) => { $ (impl DFloat for $ D { type H = $ X ; fn narrow (self) -> Self :: H { self as $ X } }) * } ; }
};
}
