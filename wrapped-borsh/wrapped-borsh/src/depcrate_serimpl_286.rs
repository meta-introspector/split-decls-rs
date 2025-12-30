// Generated macro for impl_286 (impl)
macro_rules! Depcrate_serimpl_286 {
() => {
// Module: crate::ser
// Provides: {"impl_286"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSerialize for std :: net :: SocketAddrV6 { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . ip () . serialize (writer) ? ; self . port () . serialize (writer) } }
};
}
