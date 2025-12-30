// Generated macro for impl_69 (impl)
macro_rules! Depcrate_distr_integerimpl_69 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_69"}
// Dependencies: {}
impl Distribution < u16 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u16 { rng . next_u32 () as u16 } }
};
}
