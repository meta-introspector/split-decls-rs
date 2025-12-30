// Generated macro for PositiveMeasure (trait)
macro_rules! Depcrate_algoPositiveMeasure {
() => {
// Module: crate::algo
// Provides: {"PositiveMeasure"}
// Dependencies: {}
# [doc = " Some measure of positive numbers, assuming positive"] # [doc = " float-pointing numbers"] pub trait PositiveMeasure : Measure + Copy { fn zero () -> Self ; fn max () -> Self ; }
};
}
