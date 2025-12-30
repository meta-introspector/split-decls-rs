// Generated macro for impl_85 (impl)
macro_rules! Depcrate_arbitraryimpl_85 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_85"}
// Dependencies: {}
impl < A : Arbitrary + Sync > Arbitrary for Arc < A > { fn arbitrary (g : & mut Gen) -> Arc < A > { Arc :: new (A :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Arc < A > > > { Box :: new ((* * self) . shrink () . map (Arc :: new)) } }
};
}
