// Generated macro for compute_t (function)
macro_rules! Depcrate_montycompute_t {
() => {
// Module: crate::monty
// Provides: {"compute_t"}
// Dependencies: {}
# [doc = " Compute `t = (modulus - 1) >> S`"] pub const fn compute_t < const LIMBS : usize > (modulus : & Uint < LIMBS >) -> Uint < LIMBS > { modulus . wrapping_sub (& Uint :: ONE) . wrapping_shr (compute_s (modulus)) }
};
}
