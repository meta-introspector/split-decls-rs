// Generated macro for impl_288 (impl)
macro_rules! Depcrate_bytes_mutimpl_288 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_288"}
// Dependencies: {}
impl PartialOrd < Vec < u8 > > for BytesMut { fn partial_cmp (& self , other : & Vec < u8 >) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (& other [..]) } }
};
}
