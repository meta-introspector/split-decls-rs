// Generated macro for impl_1507 (impl)
macro_rules! Depcrate_recovery_gcongestion_bbr2impl_1507 {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"impl_1507"}
// Dependencies: {}
impl < T : Ord + Clone + Copy > Limits < T > { fn min (& self) -> T { self . lo } fn apply_limits (& self , val : T) -> T { val . max (self . lo) . min (self . hi) } }
};
}
