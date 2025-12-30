// Generated macro for impl_49 (impl)
macro_rules! Depcrate_arbitraryimpl_49 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_49"}
// Dependencies: {}
impl Arbitrary for Ipv4Addr { fn arbitrary (g : & mut Gen) -> Ipv4Addr { Ipv4Addr :: new (g . random () , g . random () , g . random () , g . random ()) } }
};
}
