// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_recovery_bandwidthimpl_1072 {
() => {
// Module: crate::recovery::bandwidth
// Provides: {"impl_1072"}
// Dependencies: {}
impl std :: ops :: Mul < f64 > for Bandwidth { type Output = Bandwidth ; fn mul (self , rhs : f64) -> Self :: Output { Bandwidth { bits_per_second : (self . bits_per_second as f64 * rhs) . round () as u64 , } } }
};
}
