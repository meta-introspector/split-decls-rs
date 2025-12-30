// Generated macro for impl_1508 (impl)
macro_rules! Depcrate_recovery_gcongestion_bbr2impl_1508 {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"impl_1508"}
// Dependencies: {}
impl < T : Ord + Clone + Copy + From < u8 > > Limits < T > { pub (crate) fn no_greater_than (val : T) -> Self { Self { lo : T :: from (0) , hi : val , } } }
};
}
