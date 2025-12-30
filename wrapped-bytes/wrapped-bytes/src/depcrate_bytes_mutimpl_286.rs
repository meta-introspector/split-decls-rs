// Generated macro for impl_286 (impl)
macro_rules! Depcrate_bytes_mutimpl_286 {
() => {
// Module: crate::bytes_mut
// Provides: {"impl_286"}
// Dependencies: {}
impl PartialOrd < BytesMut > for str { fn partial_cmp (& self , other : & BytesMut) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
};
}
