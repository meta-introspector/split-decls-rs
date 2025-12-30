// Generated macro for impl_68 (impl)
macro_rules! Depcrate_distr_integerimpl_68 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_68"}
// Dependencies: {}
impl Distribution < u8 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u8 { rng . next_u32 () as u8 } }
};
}
