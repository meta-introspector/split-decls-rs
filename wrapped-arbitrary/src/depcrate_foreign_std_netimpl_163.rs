// Generated macro for impl_163 (impl)
macro_rules! Depcrate_foreign_std_netimpl_163 {
() => {
// Module: crate::foreign::std::net
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for SocketAddrV6 { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (SocketAddrV6 :: new (u . arbitrary () ? , u . arbitrary () ? , u . arbitrary () ? , u . arbitrary () ? ,)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (Ipv6Addr :: size_hint (depth) , size_hint :: and (u16 :: size_hint (depth) , size_hint :: and (u32 :: size_hint (depth) , u32 :: size_hint (depth)) ,) ,) } }
};
}
