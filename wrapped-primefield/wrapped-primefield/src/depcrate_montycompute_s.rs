// Generated macro for compute_s (function)
macro_rules! Depcrate_montycompute_s {
() => {
// Module: crate::monty
// Provides: {"compute_s"}
// Dependencies: {}
# [doc = " Compute `S = (modulus - 1).trailing_zeros()`"] const fn compute_s < const LIMBS : usize > (modulus : & Uint < LIMBS >) -> u32 { modulus . wrapping_sub (& Uint :: ONE) . trailing_zeros () }
};
}
