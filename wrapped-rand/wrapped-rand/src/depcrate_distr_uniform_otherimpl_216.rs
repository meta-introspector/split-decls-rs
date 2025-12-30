// Generated macro for impl_216 (impl)
macro_rules! Depcrate_distr_uniform_otherimpl_216 {
() => {
// Module: crate::distr::uniform::other
// Provides: {"impl_216"}
// Dependencies: {}
impl < T , const N : usize > Distribution < [T ; N] > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [T ; N] { array :: from_fn (| _ | rng . random ()) } }
};
}
