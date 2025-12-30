// Generated macro for impl_256 (impl)
macro_rules! Depcrate_hpack_test_fuzzimpl_256 {
() => {
// Module: crate::hpack::test::fuzz
// Provides: {"impl_256"}
// Dependencies: {}
impl Arbitrary for FuzzHpack { fn arbitrary (_ : & mut Gen) -> Self { FuzzHpack :: new (thread_rng () . gen ()) } }
};
}
