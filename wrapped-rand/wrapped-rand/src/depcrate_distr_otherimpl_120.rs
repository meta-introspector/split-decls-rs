// Generated macro for impl_120 (impl)
macro_rules! Depcrate_distr_otherimpl_120 {
() => {
// Module: crate::distr::other
// Provides: {"impl_120"}
// Dependencies: {}
impl < T , const N : usize > Distribution < [T ; N] > for StandardUniform where StandardUniform : Distribution < T > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> [T ; N] { array :: from_fn (| _ | rng . random ()) } }
};
}
