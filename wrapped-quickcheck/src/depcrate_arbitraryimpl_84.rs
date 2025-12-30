// Generated macro for impl_84 (impl)
macro_rules! Depcrate_arbitraryimpl_84 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_84"}
// Dependencies: {}
impl < A : Arbitrary > Arbitrary for Box < A > { fn arbitrary (g : & mut Gen) -> Box < A > { Box :: new (A :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Box < A > > > { Box :: new ((* * self) . shrink () . map (Box :: new)) } }
};
}
