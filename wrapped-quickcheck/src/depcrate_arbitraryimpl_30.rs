// Generated macro for impl_30 (impl)
macro_rules! Depcrate_arbitraryimpl_30 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_30"}
// Dependencies: {}
impl Arbitrary for bool { fn arbitrary (g : & mut Gen) -> bool { g . random () } fn shrink (& self) -> Box < dyn Iterator < Item = bool > > { if * self { single_shrinker (false) } else { empty_shrinker () } } }
};
}
