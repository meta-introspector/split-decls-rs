// Generated macro for impl_892 (impl)
macro_rules! Depcrate_packetimpl_892 {
() => {
// Module: crate::packet
// Provides: {"impl_892"}
// Dependencies: {}
impl < 'a > ConnectionId < 'a > { # [doc = " Creates a new connection ID from the given vector."] # [inline] pub const fn from_vec (cid : Vec < u8 >) -> Self { Self (ConnectionIdInner :: Vec (cid)) } # [doc = " Creates a new connection ID from the given slice."] # [inline] pub const fn from_ref (cid : & 'a [u8]) -> Self { Self (ConnectionIdInner :: Ref (cid)) } # [doc = " Returns a new owning connection ID from the given existing one."] # [inline] pub fn into_owned (self) -> ConnectionId < 'static > { ConnectionId :: from_vec (self . into ()) } }
};
}
