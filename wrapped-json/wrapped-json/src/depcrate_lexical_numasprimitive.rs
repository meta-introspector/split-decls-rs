// Generated macro for AsPrimitive (trait)
macro_rules! Depcrate_lexical_numAsPrimitive {
() => {
// Module: crate::lexical::num
// Provides: {"AsPrimitive"}
// Dependencies: {}
# [doc = " Type that can be converted to primitive with `as`."] pub trait AsPrimitive : Sized + Copy + PartialOrd { fn as_u32 (self) -> u32 ; fn as_u64 (self) -> u64 ; fn as_u128 (self) -> u128 ; fn as_usize (self) -> usize ; fn as_f32 (self) -> f32 ; fn as_f64 (self) -> f64 ; }
};
}
