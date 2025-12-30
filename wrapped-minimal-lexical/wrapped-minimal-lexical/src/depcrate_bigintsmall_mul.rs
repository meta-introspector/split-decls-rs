// Generated macro for small_mul (function)
macro_rules! Depcrate_bigintsmall_mul {
() => {
// Module: crate::bigint
// Provides: {"small_mul"}
// Dependencies: {}
# [doc = " Multiply bigint by small integer."] # [inline] pub fn small_mul (x : & mut VecType , y : Limb) -> Option < () > { let mut carry = 0 ; for xi in x . iter_mut () { let result = scalar_mul (* xi , y , carry) ; * xi = result . 0 ; carry = result . 1 ; } if carry != 0 { x . try_push (carry) ? ; } Some (()) }
};
}
