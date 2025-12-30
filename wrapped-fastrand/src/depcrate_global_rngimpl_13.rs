// Generated macro for impl_13 (impl)
macro_rules! Depcrate_global_rngimpl_13 {
() => {
// Module: crate::global_rng
// Provides: {"impl_13"}
// Dependencies: {}
impl Rng { # [doc = " Creates a new random number generator."] # [inline] pub fn new () -> Rng { try_with_rng (Rng :: fork) . unwrap_or_else (| _ | Rng :: with_seed (0x4d595df4d0f33173)) } }
};
}
