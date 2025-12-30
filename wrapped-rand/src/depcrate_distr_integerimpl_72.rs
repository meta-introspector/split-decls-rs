// Generated macro for impl_72 (impl)
macro_rules! Depcrate_distr_integerimpl_72 {
() => {
// Module: crate::distr::integer
// Provides: {"impl_72"}
// Dependencies: {}
impl Distribution < u128 > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u128 { let x = u128 :: from (rng . next_u64 ()) ; let y = u128 :: from (rng . next_u64 ()) ; (y << 64) | x } }
};
}
