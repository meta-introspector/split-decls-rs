// Generated macro for impl_117 (impl)
macro_rules! Depcrate_tests_iterimpl_117 {
() => {
// Module: crate::tests::iter
// Provides: {"impl_117"}
// Dependencies: {}
impl < B : Flags > IterNames < B > { pub (crate) fn new (flags : & B) -> Self { IterNames { flags : B :: FLAGS , idx : 0 , remaining : B :: from_bits_retain (flags . bits ()) , source : B :: from_bits_retain (flags . bits ()) , } } }
};
}
