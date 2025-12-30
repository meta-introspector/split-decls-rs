// Generated macro for impl_287 (impl)
macro_rules! Depcrate_serimpl_287 {
() => {
// Module: crate::ser
// Provides: {"impl_287"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSerialize for std :: net :: Ipv4Addr { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { writer . write_all (& self . octets ()) } }
};
}
