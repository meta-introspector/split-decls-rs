// Generated macro for impl_53 (impl)
macro_rules! Depcrate_arbitraryimpl_53 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_53"}
// Dependencies: {}
impl Arbitrary for SocketAddrV6 { fn arbitrary (g : & mut Gen) -> SocketAddrV6 { SocketAddrV6 :: new (Arbitrary :: arbitrary (g) , g . random () , g . random () , g . random () ,) } }
};
}
