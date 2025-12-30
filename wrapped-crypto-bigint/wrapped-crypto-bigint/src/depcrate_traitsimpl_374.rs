// Generated macro for impl_374 (impl)
macro_rules! Depcrate_traitsimpl_374 {
() => {
// Module: crate::traits
// Provides: {"impl_374"}
// Dependencies: {}
impl < T , Exponent , BasesAndExponents > MultiExponentiate < Exponent , BasesAndExponents > for T where T : MultiExponentiateBoundedExp < Exponent , BasesAndExponents > , Exponent : Bounded , BasesAndExponents : AsRef < [(Self , Exponent)] > + ? Sized , { fn multi_exponentiate (bases_and_exponents : & BasesAndExponents) -> Self { Self :: multi_exponentiate_bounded_exp (bases_and_exponents , Exponent :: BITS) } }
};
}
