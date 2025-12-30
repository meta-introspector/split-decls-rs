// Generated macro for impl_unit_measure (macro)
macro_rules! Depcrate_algoimpl_unit_measure {
() => {
// Module: crate::algo
// Provides: {"impl_unit_measure"}
// Dependencies: {}
macro_rules ! impl_unit_measure (($ ($ t : ident) ,*) => { $ (impl UnitMeasure for $ t { fn zero () -> Self { 0 as $ t } fn one () -> Self { 1 as $ t } fn from_usize (nb : usize) -> Self { nb as $ t } fn default_tol () -> Self { 1e-6 as $ t } fn from_f32 (val : f32) -> Self { val as $ t } fn from_f64 (val : f64) -> Self { val as $ t } }) * }) ;
};
}
