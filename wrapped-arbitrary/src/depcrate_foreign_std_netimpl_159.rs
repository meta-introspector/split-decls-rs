// Generated macro for impl_159 (impl)
macro_rules! Depcrate_foreign_std_netimpl_159 {
() => {
// Module: crate::foreign::std::net
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a > Arbitrary < 'a > for Ipv4Addr { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (Ipv4Addr :: from (u32 :: arbitrary (u) ?)) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { (4 , Some (4)) } }
};
}
