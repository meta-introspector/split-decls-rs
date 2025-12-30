// Generated macro for impl_217 (impl)
macro_rules! Depcrate_distr_uniform_otherimpl_217 {
() => {
// Module: crate::distr::uniform::other
// Provides: {"impl_217"}
// Dependencies: {}
impl < T > Distribution < Wrapping < T > > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Wrapping < T > { Wrapping (rng . random ()) } }
};
}
