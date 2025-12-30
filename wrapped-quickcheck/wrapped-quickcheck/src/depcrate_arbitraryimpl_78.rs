// Generated macro for impl_78 (impl)
macro_rules! Depcrate_arbitraryimpl_78 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for RangeInclusive < T > { fn arbitrary (g : & mut Gen) -> RangeInclusive < T > { Arbitrary :: arbitrary (g) ..= Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = RangeInclusive < T > > > { Box :: new ((self . start () . clone () , self . end () . clone ()) . shrink () . map (| (s , e) | s ..= e) ,) } }
};
}
