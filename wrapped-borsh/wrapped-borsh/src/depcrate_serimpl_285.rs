// Generated macro for impl_285 (impl)
macro_rules! Depcrate_serimpl_285 {
() => {
// Module: crate::ser
// Provides: {"impl_285"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSerialize for std :: net :: SocketAddrV4 { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { self . ip () . serialize (writer) ? ; self . port () . serialize (writer) } }
};
}
