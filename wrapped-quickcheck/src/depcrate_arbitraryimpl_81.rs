// Generated macro for impl_81 (impl)
macro_rules! Depcrate_arbitraryimpl_81 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_81"}
// Dependencies: {}
impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeToInclusive < T > { fn arbitrary (g : & mut Gen) -> RangeToInclusive < T > { ..= Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = RangeToInclusive < T > > > { Box :: new (self . end . clone () . shrink () . map (| end | ..= end)) } }
};
}
