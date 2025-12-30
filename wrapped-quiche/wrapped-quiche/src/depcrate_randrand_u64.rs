// Generated macro for rand_u64 (function)
macro_rules! Depcrate_randrand_u64 {
() => {
// Module: crate::rand
// Provides: {"rand_u64"}
// Dependencies: {}
pub fn rand_u64 () -> u64 { let mut buf = [0 ; 8] ; rand_bytes (& mut buf) ; u64 :: from_ne_bytes (buf) }
};
}
