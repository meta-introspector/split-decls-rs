// Generated macro for impl_288 (impl)
macro_rules! Depcrate_serimpl_288 {
() => {
// Module: crate::ser
// Provides: {"impl_288"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSerialize for std :: net :: Ipv6Addr { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { writer . write_all (& self . octets ()) } }
};
}
