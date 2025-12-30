// Generated macro for impl_37 (impl)
macro_rules! Depcrate_arbitraryimpl_37 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_37"}
// Dependencies: {}
impl < A : Arbitrary > Arbitrary for Vec < A > { fn arbitrary (g : & mut Gen) -> Vec < A > { let size = { let s = g . size () ; g . random_range (0 .. s) } ; (0 .. size) . map (| _ | A :: arbitrary (g)) . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = Vec < A > > > { VecShrinker :: new (self . clone ()) } }
};
}
