// Generated macro for impl_46 (impl)
macro_rules! Depcrate_arbitraryimpl_46 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_46"}
// Dependencies: {}
impl < T : Arbitrary > Arbitrary for LinkedList < T > { fn arbitrary (g : & mut Gen) -> LinkedList < T > { let vec : Vec < T > = Arbitrary :: arbitrary (g) ; vec . into_iter () . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = LinkedList < T > > > { let vec : Vec < T > = self . clone () . into_iter () . collect () ; Box :: new (vec . shrink () . map (| v | v . into_iter () . collect :: < LinkedList < T > > ()) ,) } }
};
}
