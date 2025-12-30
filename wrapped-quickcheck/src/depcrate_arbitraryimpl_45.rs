// Generated macro for impl_45 (impl)
macro_rules! Depcrate_arbitraryimpl_45 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_45"}
// Dependencies: {}
impl < T : Arbitrary + Eq + Hash , S : BuildHasher + Default + Clone + 'static > Arbitrary for HashSet < T , S > { fn arbitrary (g : & mut Gen) -> Self { let vec : Vec < T > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = Self > > { let vec : Vec < T > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < Self > ())) } }
};
}
