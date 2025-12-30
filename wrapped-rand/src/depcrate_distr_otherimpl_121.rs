// Generated macro for impl_121 (impl)
macro_rules! Depcrate_distr_otherimpl_121 {
() => {
// Module: crate::distr::other
// Provides: {"impl_121"}
// Dependencies: {}
impl < T > Distribution < Wrapping < T > > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Wrapping < T > { Wrapping (rng . random ()) } }
};
}
