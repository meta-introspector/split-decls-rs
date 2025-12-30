// Generated macro for rand_bytes (function)
macro_rules! Depcrate_randrand_bytes {
() => {
// Module: crate::rand
// Provides: {"rand_bytes"}
// Dependencies: {}
pub fn rand_bytes (buf : & mut [u8]) { unsafe { RAND_bytes (buf . as_mut_ptr () , buf . len ()) ; } }
};
}
