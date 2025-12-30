// Generated macro for FloatMeasure (trait)
macro_rules! Depcrate_algoFloatMeasure {
() => {
// Module: crate::algo
// Provides: {"FloatMeasure"}
// Dependencies: {}
# [doc = " A floating-point measure."] pub trait FloatMeasure : Measure + Copy { fn zero () -> Self ; fn infinite () -> Self ; fn from_f32 (val : f32) -> Self ; fn from_f64 (val : f64) -> Self ; }
};
}
