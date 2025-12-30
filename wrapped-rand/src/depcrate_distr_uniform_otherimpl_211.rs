// Generated macro for impl_211 (impl)
macro_rules! Depcrate_distr_uniform_otherimpl_211 {
() => {
// Module: crate::distr::uniform::other
// Provides: {"impl_211"}
// Dependencies: {}
impl Distribution < bool > for StandardUniform { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> bool { (rng . next_u32 () as i32) < 0 } }
};
}
