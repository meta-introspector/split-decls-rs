// Generated macro for mul_high_u64 (function)
macro_rules! Depcratemul_high_u64 {
() => {
// Module: crate
// Provides: {"mul_high_u64"}
// Dependencies: {}
# [doc = " Computes `(a * b) >> 64`."] # [inline] fn mul_high_u64 (a : u64 , b : u64) -> u64 { (((a as u128) * (b as u128)) >> 64) as u64 }
};
}
