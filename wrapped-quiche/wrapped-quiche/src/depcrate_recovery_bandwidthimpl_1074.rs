// Generated macro for impl_1074 (impl)
macro_rules! Depcrate_recovery_bandwidthimpl_1074 {
() => {
// Module: crate::recovery::bandwidth
// Provides: {"impl_1074"}
// Dependencies: {}
impl std :: ops :: Sub < Bandwidth > for Bandwidth { type Output = Option < Bandwidth > ; fn sub (self , rhs : Bandwidth) -> Self :: Output { self . bits_per_second . checked_sub (rhs . bits_per_second) . map (| bps | Bandwidth { bits_per_second : bps , }) } }
};
}
