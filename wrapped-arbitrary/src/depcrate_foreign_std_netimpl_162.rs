// Generated macro for impl_162 (impl)
macro_rules! Depcrate_foreign_std_netimpl_162 {
() => {
// Module: crate::foreign::std::net
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for SocketAddrV4 { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (SocketAddrV4 :: new (u . arbitrary () ? , u . arbitrary () ?)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (Ipv4Addr :: size_hint (depth) , u16 :: size_hint (depth)) } }
};
}
