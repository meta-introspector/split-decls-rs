// Generated macro for impl_289 (impl)
macro_rules! Depcrate_serimpl_289 {
() => {
// Module: crate::ser
// Provides: {"impl_289"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSerialize for std :: net :: IpAddr { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { match self { std :: net :: IpAddr :: V4 (ipv4) => { writer . write_all (& 0u8 . to_le_bytes ()) ? ; ipv4 . serialize (writer) } std :: net :: IpAddr :: V6 (ipv6) => { writer . write_all (& 1u8 . to_le_bytes ()) ? ; ipv6 . serialize (writer) } } } }
};
}
