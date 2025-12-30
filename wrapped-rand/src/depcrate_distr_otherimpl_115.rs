// Generated macro for impl_115 (impl)
macro_rules! Depcrate_distr_otherimpl_115 {
() => {
// Module: crate::distr::other
// Provides: {"impl_115"}
// Dependencies: {}
impl Distribution < bool > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> bool { (rng . next_u32 () as i32) < 0 } }
};
}
