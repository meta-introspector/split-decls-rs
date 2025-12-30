// Generated macro for BoundedMeasure (trait)
macro_rules! Depcrate_algoBoundedMeasure {
() => {
// Module: crate::algo
// Provides: {"BoundedMeasure"}
// Dependencies: {}
pub trait BoundedMeasure : Measure + core :: ops :: Sub < Self , Output = Self > { fn min () -> Self ; fn max () -> Self ; fn overflowing_add (self , rhs : Self) -> (Self , bool) ; fn from_f32 (val : f32) -> Self ; fn from_f64 (val : f64) -> Self ; }
};
}
