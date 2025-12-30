// Generated macro for gen_range (function)
macro_rules! Depcrate_sortgen_range {
() => {
// Module: crate::sort
// Provides: {"gen_range"}
// Dependencies: {}
fn gen_range < R : RngCore > (rng : & mut R , min : usize , max : usize) -> usize { let range = max - min ; min + (rng . next_u64 () as usize % range) }
};
}
