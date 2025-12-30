// Generated macro for Match (struct)
macro_rules! Depcrate_simd_accel_teddy128Match {
() => {
// Module: crate::simd_accel::teddy128
// Provides: {"Match"}
// Dependencies: {}
# [doc = " Match reports match information."] # [derive (Debug , Clone)] pub struct Match { # [doc = " The index of the pattern that matched. The index is in correspondence"] # [doc = " with the order of the patterns given at construction."] pub pat : usize , # [doc = " The start byte offset of the match."] pub start : usize , # [doc = " The end byte offset of the match. This is always `start + pat.len()`."] pub end : usize , }
};
}
