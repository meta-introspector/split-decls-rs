// Generated macro for generate_park_miller_lehmer_random_number (function)
macro_rules! Depcrate_syscalls_entropygenerate_park_miller_lehmer_random_number {
() => {
// Module: crate::syscalls::entropy
// Provides: {"generate_park_miller_lehmer_random_number"}
// Dependencies: {}
fn generate_park_miller_lehmer_random_number () -> u32 { let mut seed = PARK_MILLER_LEHMER_SEED . lock () ; let random = ((u64 :: from (* seed) * 48271) % RAND_MAX) as u32 ; * seed = random ; random }
};
}
