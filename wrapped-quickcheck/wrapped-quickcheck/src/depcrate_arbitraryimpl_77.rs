// Generated macro for impl_77 (impl)
macro_rules! Depcrate_arbitraryimpl_77 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_77"}
// Dependencies: {}
impl < T : Arbitrary + Clone + PartialOrd > Arbitrary for Range < T > { fn arbitrary (g : & mut Gen) -> Range < T > { Arbitrary :: arbitrary (g) .. Arbitrary :: arbitrary (g) } fn shrink (& self) -> Box < dyn Iterator < Item = Range < T > > > { Box :: new ((self . start . clone () , self . end . clone ()) . shrink () . map (| (s , e) | s .. e) ,) } }
};
}
