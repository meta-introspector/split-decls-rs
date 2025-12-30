// Generated macro for impl_895 (impl)
macro_rules! Depcrate_packetimpl_895 {
() => {
// Module: crate::packet
// Provides: {"impl_895"}
// Dependencies: {}
impl From < ConnectionId < '_ > > for Vec < u8 > { # [inline] fn from (id : ConnectionId < '_ >) -> Self { match id . 0 { ConnectionIdInner :: Vec (cid) => cid , ConnectionIdInner :: Ref (cid) => cid . to_vec () , } } }
};
}
