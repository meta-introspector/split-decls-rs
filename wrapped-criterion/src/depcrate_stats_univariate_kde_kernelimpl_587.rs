// Generated macro for impl_587 (impl)
macro_rules! Depcrate_stats_univariate_kde_kernelimpl_587 {
() => {
// Module: crate::stats::univariate::kde::kernel
// Provides: {"impl_587"}
// Dependencies: {}
impl < A > Kernel < A > for Gaussian where A : Float , { fn evaluate (& self , x : A) -> A { use std :: f32 :: consts :: PI ; (x . powi (2) . exp () * A :: cast (2. * PI)) . sqrt () . recip () } }
};
}
