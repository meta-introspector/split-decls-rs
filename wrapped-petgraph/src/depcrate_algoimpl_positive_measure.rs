// Generated macro for impl_positive_measure (macro)
macro_rules! Depcrate_algoimpl_positive_measure {
() => {
// Module: crate::algo
// Provides: {"impl_positive_measure"}
// Dependencies: {}
macro_rules ! impl_positive_measure (($ ($ t : ident) ,*) => { $ (impl PositiveMeasure for $ t { fn zero () -> Self { 0 as $ t } fn max () -> Self { $ t :: MAX } }) * }) ;
};
}
