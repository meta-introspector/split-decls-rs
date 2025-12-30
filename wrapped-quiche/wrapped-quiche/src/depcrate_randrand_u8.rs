// Generated macro for rand_u8 (function)
macro_rules! Depcrate_randrand_u8 {
() => {
// Module: crate::rand
// Provides: {"rand_u8"}
// Dependencies: {}
pub fn rand_u8 () -> u8 { let mut buf = [0 ; 1] ; rand_bytes (& mut buf) ; buf [0] }
};
}
