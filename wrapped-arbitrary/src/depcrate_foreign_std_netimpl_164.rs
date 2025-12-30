// Generated macro for impl_164 (impl)
macro_rules! Depcrate_foreign_std_netimpl_164 {
() => {
// Module: crate::foreign::std::net
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for SocketAddr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { if u . arbitrary () ? { Ok (SocketAddr :: V4 (u . arbitrary () ?)) } else { Ok (SocketAddr :: V6 (u . arbitrary () ?)) } } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (bool :: size_hint (depth) , size_hint :: or (SocketAddrV4 :: size_hint (depth) , SocketAddrV6 :: size_hint (depth) ,) ,) } }
};
}
