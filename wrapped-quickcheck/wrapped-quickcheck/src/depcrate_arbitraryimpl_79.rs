// Generated macro for impl_79 (impl)
macro_rules! Depcrate_arbitraryimpl_79 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_79"}
// Dependencies: {}
impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeFrom < T > { fn arbitrary (g : & mut Gen) -> RangeFrom < T > { Arbitrary :: arbitrary (g) .. } fn shrink (& self) -> Box < dyn Iterator < Item = RangeFrom < T > > > { Box :: new (self . start . clone () . shrink () . map (| start | start ..)) } }
};
}
