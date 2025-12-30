// Generated macro for impl_3035 (impl)
macro_rules! Depcrate_proptestimpl_3035 {
() => {
// Module: crate::proptest
// Provides: {"impl_3035"}
// Dependencies: {}
impl < NParameters , C > Default for MatrixParameters < NParameters , Dyn , C > where NParameters : Default , C : DimName , { fn default () -> Self { Self { rows : dynamic_dim_range () , cols : DimRange :: from (C :: name ()) , value_parameters : NParameters :: default () , } } }
};
}
