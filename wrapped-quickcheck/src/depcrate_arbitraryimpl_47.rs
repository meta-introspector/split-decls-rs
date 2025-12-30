// Generated macro for impl_47 (impl)
macro_rules! Depcrate_arbitraryimpl_47 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : Arbitrary > Arbitrary for VecDeque < T > { fn arbitrary (g : & mut Gen) -> VecDeque < T > { let vec : Vec < T > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = VecDeque < T > > > { let vec : Vec < T > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < VecDeque < T > > ())) } }
};
}
