// Generated macro for impl_71 (impl)
macro_rules! Depcrate_distr_integerimpl_71 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_71"}
// Dependencies: {}
impl Distribution < u64 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u64 { rng . next_u64 () } }
};
}
