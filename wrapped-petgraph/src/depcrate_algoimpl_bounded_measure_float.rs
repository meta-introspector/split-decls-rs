// Generated macro for impl_bounded_measure_float (macro)
macro_rules! Depcrate_algoimpl_bounded_measure_float {
() => {
// Module: crate::algo
// Provides: {"impl_bounded_measure_float"}
// Dependencies: {}
macro_rules ! impl_bounded_measure_float (($ ($ t : ident) ,*) => { $ (impl BoundedMeasure for $ t { fn min () -> Self { $ t :: MIN } fn max () -> Self { $ t :: MAX } fn overflowing_add (self , rhs : Self) -> (Self , bool) { let overflow = self > Self :: default () && rhs > Self :: default () && self > $ t :: MAX - rhs ; let underflow = ! overflow && self < Self :: default () && rhs < Self :: default () && self < $ t :: MIN - rhs ; (self + rhs , overflow || underflow) } fn from_f32 (val : f32) -> Self { val as $ t } fn from_f64 (val : f64) -> Self { val as $ t } }) * } ;) ;
};
}
