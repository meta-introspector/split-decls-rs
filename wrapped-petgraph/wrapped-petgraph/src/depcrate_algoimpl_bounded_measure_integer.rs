// Generated macro for impl_bounded_measure_integer (macro)
macro_rules! Depcrate_algoimpl_bounded_measure_integer {
() => {
// Module: crate::algo
// Provides: {"impl_bounded_measure_integer"}
// Dependencies: {}
macro_rules ! impl_bounded_measure_integer (($ ($ t : ident) ,*) => { $ (impl BoundedMeasure for $ t { fn min () -> Self { $ t :: MIN } fn max () -> Self { $ t :: MAX } fn overflowing_add (self , rhs : Self) -> (Self , bool) { self . overflowing_add (rhs) } fn from_f32 (val : f32) -> Self { val as $ t } fn from_f64 (val : f64) -> Self { val as $ t } }) * } ;) ;
};
}
