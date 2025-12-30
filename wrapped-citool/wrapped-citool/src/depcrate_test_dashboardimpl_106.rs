// Generated macro for impl_106 (impl)
macro_rules! Depcrate_test_dashboardimpl_106 {
() => {
// Module: crate::test_dashboard
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a > TestGroup < 'a > { fn test_count (& self) -> u64 { let root = self . root_tests . len () as u64 ; self . groups . iter () . map (| (_ , group) | group . test_count ()) . sum :: < u64 > () + root } }
};
}
