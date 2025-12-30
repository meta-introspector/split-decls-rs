// Generated macro for Kernel (trait)
macro_rules! Depcrate_stats_univariate_kde_kernelKernel {
() => {
// Module: crate::stats::univariate::kde::kernel
// Provides: {"Kernel"}
// Dependencies: {}
# [doc = " Kernel function"] pub trait Kernel < A > : Copy + Sync where A : Float , { # [doc = " Apply the kernel function to the given x-value."] fn evaluate (& self , x : A) -> A ; }
};
}
