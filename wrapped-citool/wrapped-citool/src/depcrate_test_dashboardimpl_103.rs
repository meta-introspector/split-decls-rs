// Generated macro for impl_103 (impl)
macro_rules! Depcrate_test_dashboardimpl_103 {
() => {
// Module: crate::test_dashboard
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a > Test < 'a > { # [doc = " If this is a test without revisions, it will have a single entry in `revisions` with"] # [doc = " an empty string as the revision name."] fn single_test (& self) -> Option < & TestResults < 'a > > { if self . revisions . len () == 1 { self . revisions . iter () . next () . take_if (| e | e . 0 . is_empty ()) . map (| e | e . 1) } else { None } } }
};
}
