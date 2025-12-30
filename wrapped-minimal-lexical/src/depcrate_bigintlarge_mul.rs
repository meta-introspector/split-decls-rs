// Generated macro for large_mul (function)
macro_rules! Depcrate_bigintlarge_mul {
() => {
// Module: crate::bigint
// Provides: {"large_mul"}
// Dependencies: {}
# [doc = " Multiply bigint by bigint using grade-school multiplication algorithm."] # [inline (always)] pub fn large_mul (x : & mut VecType , y : & [Limb]) -> Option < () > { if y . len () == 1 { small_mul (x , y [0]) ? ; } else { * x = long_mul (y , x) ? ; } Some (()) }
};
}
