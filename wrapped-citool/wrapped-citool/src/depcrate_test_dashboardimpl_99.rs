// Generated macro for impl_99 (impl)
macro_rules! Depcrate_test_dashboardimpl_99 {
() => {
// Module: crate::test_dashboard
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'a > TestSuites < 'a > { fn test_count (& self) -> u64 { self . suites . iter () . map (| suite | suite . group . test_count ()) . sum :: < u64 > () } }
};
}
