// Generated macro for impl_76 (impl)
macro_rules! Depcrate_arbitraryimpl_76 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : Arbitrary > Arbitrary for Bound < T > { fn arbitrary (g : & mut Gen) -> Bound < T > { match g . random_range (0 .. 3) { 0 => Bound :: Included (T :: arbitrary (g)) , 1 => Bound :: Excluded (T :: arbitrary (g)) , _ => Bound :: Unbounded , } } fn shrink (& self) -> Box < dyn Iterator < Item = Bound < T > > > { match * self { Bound :: Included (ref x) => { Box :: new (x . shrink () . map (Bound :: Included)) } Bound :: Excluded (ref x) => { Box :: new (x . shrink () . map (Bound :: Excluded)) } Bound :: Unbounded => empty_shrinker () , } } }
};
}
