// Generated macro for impl_371 (impl)
macro_rules! Depcrate_traitsimpl_371 {
() => {
// Module: crate::traits
// Provides: {"impl_371"}
// Dependencies: {}
impl < T : PowBoundedExp < Exponent > , Exponent : Bounded > Pow < Exponent > for T { fn pow (& self , exponent : & Exponent) -> Self { self . pow_bounded_exp (exponent , Exponent :: BITS) } }
};
}
