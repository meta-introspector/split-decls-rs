// Generated macro for impl_294 (impl)
macro_rules! Depcrate_bytes_mutimpl_294 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_294"}
// Dependencies: {}
impl PartialOrd < BytesMut > for String { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
};
}
