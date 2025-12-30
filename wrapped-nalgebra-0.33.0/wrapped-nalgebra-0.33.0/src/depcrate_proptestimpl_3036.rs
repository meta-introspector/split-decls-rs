// Generated macro for impl_3036 (impl)
macro_rules! Depcrate_proptestimpl_3036 {
() => {
// Module: crate::proptest
// Provides: {"impl_3036"}
// Dependencies: {}
impl < NParameters > Default for MatrixParameters < NParameters , Dyn , Dyn > where NParameters : Default , { fn default () -> Self { Self { rows : dynamic_dim_range () , cols : dynamic_dim_range () , value_parameters : NParameters :: default () , } } }
};
}
