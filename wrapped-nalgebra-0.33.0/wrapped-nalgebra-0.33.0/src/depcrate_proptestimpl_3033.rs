// Generated macro for impl_3033 (impl)
macro_rules! Depcrate_proptestimpl_3033 {
() => {
// Module: crate::proptest
// Provides: {"impl_3033"}
// Dependencies: {}
impl < NParameters , R , C > Default for MatrixParameters < NParameters , R , C > where NParameters : Default , R : DimName , C : DimName , { fn default () -> Self { Self { rows : DimRange :: from (R :: name ()) , cols : DimRange :: from (C :: name ()) , value_parameters : NParameters :: default () , } } }
};
}
