// Generated macro for impl_134 (impl)
macro_rules! Depcrate_tests_tokenimpl_134 {
() => {
// Module: crate::tests::token
// Provides: {"impl_134"}
// Dependencies: {}
impl FakeTimeSource { pub (super) fn new () -> Self { Self (Mutex :: new (SystemTime :: now ())) } pub (super) fn advance (& self , dur : Duration) { * self . 0 . lock () . unwrap () += dur ; } }
};
}
