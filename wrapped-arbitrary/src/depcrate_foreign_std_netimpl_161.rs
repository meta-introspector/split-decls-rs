// Generated macro for impl_161 (impl)
macro_rules! Depcrate_foreign_std_netimpl_161 {
() => {
// Module: crate::foreign::std::net
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for IpAddr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { if u . arbitrary () ? { Ok (IpAddr :: V4 (u . arbitrary () ?)) } else { Ok (IpAddr :: V6 (u . arbitrary () ?)) } } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (bool :: size_hint (depth) , size_hint :: or (Ipv4Addr :: size_hint (depth) , Ipv6Addr :: size_hint (depth)) ,) } }
};
}
