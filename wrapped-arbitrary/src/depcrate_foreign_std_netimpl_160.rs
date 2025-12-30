// Generated macro for impl_160 (impl)
macro_rules! Depcrate_foreign_std_netimpl_160 {
() => {
// Module: crate::foreign::std::net
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for Ipv6Addr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (Ipv6Addr :: from (u128 :: arbitrary (u) ?)) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (16 , Some (16)) } }
};
}
