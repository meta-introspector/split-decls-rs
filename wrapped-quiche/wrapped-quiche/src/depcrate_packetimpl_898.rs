// Generated macro for impl_898 (impl)
macro_rules! Depcrate_packetimpl_898 {
() => {
// Module: crate::packet
// Provides: {"impl_898"}
// Dependencies: {}
impl AsRef < [u8] > for ConnectionId < '_ > { # [inline] fn as_ref (& self) -> & [u8] { match & self . 0 { ConnectionIdInner :: Vec (v) => v . as_ref () , ConnectionIdInner :: Ref (v) => v , } } }
};
}
