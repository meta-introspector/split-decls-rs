// Generated macro for impl_1076 (impl)
macro_rules! Depcrate_recovery_bandwidthimpl_1076 {
() => {
// Module: crate::recovery::bandwidth
// Provides: {"impl_1076"}
// Dependencies: {}
impl std :: ops :: Mul < Duration > for Bandwidth { type Output = u64 ; fn mul (self , rhs : Duration) -> Self :: Output { self . to_bytes_per_period (rhs) } }
};
}
