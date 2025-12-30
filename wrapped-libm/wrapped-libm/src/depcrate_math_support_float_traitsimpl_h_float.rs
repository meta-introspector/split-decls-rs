// Generated macro for impl_h_float (macro)
macro_rules! Depcrate_math_support_float_traitsimpl_h_float {
() => {
// Module: crate::math::support::float_traits
// Provides: {"impl_h_float"}
// Dependencies: {}
macro_rules ! impl_h_float { ($ ($ H : ident $ X : ident) ,*) => { $ (impl HFloat for $ H { type D = $ X ; fn widen (self) -> Self :: D { self as $ X } }) * } ; }
};
}
