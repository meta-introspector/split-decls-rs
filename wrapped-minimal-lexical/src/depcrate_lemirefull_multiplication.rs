// Generated macro for full_multiplication (function)
macro_rules! Depcrate_lemirefull_multiplication {
() => {
// Module: crate::lemire
// Provides: {"full_multiplication"}
// Dependencies: {}
# [inline] fn full_multiplication (a : u64 , b : u64) -> (u64 , u64) { let r = (a as u128) * (b as u128) ; (r as u64 , (r >> 64) as u64) }
};
}
