// Generated macro for impl_3034 (impl)
macro_rules! Depcrate_proptestimpl_3034 {
() => {
// Module: crate::proptest
// Provides: {"impl_3034"}
// Dependencies: {}
impl < NParameters , R > Default for MatrixParameters < NParameters , R , Dyn > where NParameters : Default , R : DimName , { fn default () -> Self { Self { rows : DimRange :: from (R :: name ()) , cols : dynamic_dim_range () , value_parameters : NParameters :: default () , } } }
};
}
