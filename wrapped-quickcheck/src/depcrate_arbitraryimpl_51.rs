// Generated macro for impl_51 (impl)
macro_rules! Depcrate_arbitraryimpl_51 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_51"}
// Dependencies: {}
impl Arbitrary for SocketAddr { fn arbitrary (g : & mut Gen) -> SocketAddr { SocketAddr :: new (Arbitrary :: arbitrary (g) , g . random ()) } }
};
}
