// Generated macro for impl_135 (impl)
macro_rules! Depcrate_tests_tokenimpl_135 {
() => {
// Module: crate::tests::token
// Provides: {"impl_135"}
// Dependencies: {}
impl TimeSource for FakeTimeSource { fn now (& self) -> SystemTime { * self . 0 . lock () . unwrap () } }
};
}
