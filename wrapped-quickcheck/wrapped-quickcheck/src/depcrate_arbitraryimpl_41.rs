// Generated macro for impl_41 (impl)
macro_rules! Depcrate_arbitraryimpl_41 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_41"}
// Dependencies: {}
impl < K : Arbitrary + Ord , V : Arbitrary > Arbitrary for BTreeMap < K , V > { fn arbitrary (g : & mut Gen) -> BTreeMap < K , V > { let vec : Vec < (K , V) > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = BTreeMap < K , V > > > { let vec : Vec < (K , V) > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < BTreeMap < K , V > > ()) ,) } }
};
}
