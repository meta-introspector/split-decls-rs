// Generated macro for gen_data (function)
macro_rules! Depcrate_testsgen_data {
() => {
// Module: crate::tests
// Provides: {"gen_data"}
// Dependencies: {}
fn gen_data (size : usize , seed : u64) -> Vec < u8 > { let mut rng : StdRng = SeedableRng :: seed_from_u64 (seed) ; let mut buf = vec ! [0 ; size] ; rng . fill_bytes (& mut buf) ; buf }
};
}
