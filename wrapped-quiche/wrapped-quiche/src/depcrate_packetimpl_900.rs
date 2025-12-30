// Generated macro for impl_900 (impl)
macro_rules! Depcrate_packetimpl_900 {
() => {
// Module: crate::packet
// Provides: {"impl_900"}
// Dependencies: {}
impl std :: ops :: Deref for ConnectionId < '_ > { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { match & self . 0 { ConnectionIdInner :: Vec (v) => v . as_ref () , ConnectionIdInner :: Ref (v) => v , } } }
};
}
