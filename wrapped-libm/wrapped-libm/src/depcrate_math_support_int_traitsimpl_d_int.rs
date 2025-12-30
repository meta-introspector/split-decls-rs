// Generated macro for impl_d_int (macro)
macro_rules! Depcrate_math_support_int_traitsimpl_d_int {
() => {
// Module: crate::math::support::int_traits
// Provides: {"impl_d_int"}
// Dependencies: {}
macro_rules ! impl_d_int { ($ ($ X : ident $ D : ident) ,*) => { $ (impl DInt for $ D { type H = $ X ; fn lo (self) -> Self :: H { self as $ X } fn hi (self) -> Self :: H { (self >> <$ X as MinInt >:: BITS) as $ X } }) * } ; }
};
}
