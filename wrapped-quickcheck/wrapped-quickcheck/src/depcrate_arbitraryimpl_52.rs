// Generated macro for impl_52 (impl)
macro_rules! Depcrate_arbitraryimpl_52 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_52"}
// Dependencies: {}
impl Arbitrary for SocketAddrV4 { fn arbitrary (g : & mut Gen) -> SocketAddrV4 { SocketAddrV4 :: new (Arbitrary :: arbitrary (g) , g . random ()) } }
};
}
