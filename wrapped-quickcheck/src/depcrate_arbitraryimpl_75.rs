// Generated macro for impl_75 (impl)
macro_rules! Depcrate_arbitraryimpl_75 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : Arbitrary > Arbitrary for Wrapping < T > { fn arbitrary (g : & mut Gen) -> Wrapping < T > { Wrapping (T :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = Wrapping < T > > > { Box :: new (self . 0 . shrink () . map (Wrapping)) } }
};
}
