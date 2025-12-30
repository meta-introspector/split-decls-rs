// Generated macro for impl_56 (impl)
macro_rules! Depcrate_arbitraryimpl_56 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_56"}
// Dependencies: {}
impl Arbitrary for String { fn arbitrary (g : & mut Gen) -> String { let size = { let s = g . size () ; g . random_range (0 .. s) } ; (0 .. size) . map (| _ | char :: arbitrary (g)) . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = String > > { let chars : Vec < char > = self . chars () . collect () ; Box :: new (chars . shrink () . map (| x | x . into_iter () . collect :: < String > ())) } }
};
}
