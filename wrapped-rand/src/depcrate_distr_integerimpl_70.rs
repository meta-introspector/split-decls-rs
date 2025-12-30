// Generated macro for impl_70 (impl)
macro_rules! Depcrate_distr_integerimpl_70 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_70"}
// Dependencies: {}
impl Distribution < u32 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u32 { rng . next_u32 () } }
};
}
