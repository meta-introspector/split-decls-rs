// Generated macro for impl_142 (impl)
macro_rules! Depcrate_bytesimpl_142 {
() => {
// Module: crate::bytes
// Provides: {"impl_142"}
// Dependencies: {}
impl PartialOrd < Bytes > for str { fn partial_cmp (& self , other : & Bytes) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
};
}
