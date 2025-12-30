// Generated macro for impl_284 (impl)
macro_rules! Depcrate_serimpl_284 {
() => {
// Module: crate::ser
// Provides: {"impl_284"}
// Dependencies: {}
# [cfg (feature = "std")] impl BorshSerialize for std :: net :: SocketAddr { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { match * self { std :: net :: SocketAddr :: V4 (ref addr) => { 0u8 . serialize (writer) ? ; addr . serialize (writer) } std :: net :: SocketAddr :: V6 (ref addr) => { 1u8 . serialize (writer) ? ; addr . serialize (writer) } } } }
};
}
