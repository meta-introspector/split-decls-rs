// Generated macro for mul_high_u32 (function)
macro_rules! Depcratemul_high_u32 {
() => {
// Module: crate
// Provides: {"mul_high_u32"}
// Dependencies: {}
# [doc = " Computes `(a * b) >> 32`."] # [inline] fn mul_high_u32 (a : u32 , b : u32) -> u32 { (((a as u64) * (b as u64)) >> 32) as u32 }
};
}
