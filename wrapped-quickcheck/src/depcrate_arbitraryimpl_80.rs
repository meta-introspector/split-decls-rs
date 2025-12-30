// Generated macro for impl_80 (impl)
macro_rules! Depcrate_arbitraryimpl_80 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_80"}
// Dependencies: {}
impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeTo < T > { fn arbitrary (g : & mut Gen) -> RangeTo < T > { .. Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = RangeTo < T > > > { Box :: new (self . end . clone () . shrink () . map (| end | .. end)) } }
};
}
