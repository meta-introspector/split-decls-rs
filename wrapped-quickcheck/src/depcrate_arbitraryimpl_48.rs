// Generated macro for impl_48 (impl)
macro_rules! Depcrate_arbitraryimpl_48 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_48"}
// Dependencies: {}
impl Arbitrary for IpAddr { fn arbitrary (g : & mut Gen) -> IpAddr { let ipv4 : bool = g . random () ; if ipv4 { IpAddr :: V4 (Arbitrary :: arbitrary (g)) } else { IpAddr :: V6 (Arbitrary :: arbitrary (g)) } } }
};
}
