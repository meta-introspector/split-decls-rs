// Generated macro for impl_292 (impl)
macro_rules! Depcrate_bytes_mutimpl_292 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_292"}
// Dependencies: {}
impl PartialOrd < String > for BytesMut { fn partial_cmp (& self , other : & String) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (other . as_bytes ()) } }
};
}
