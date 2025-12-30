// Generated macro for impl_150 (impl)
macro_rules! Depcrate_bytesimpl_150 {
() => {
// Module: crate::bytes
// Provides: {"impl_150"}
// Dependencies: {}
impl PartialOrd < Bytes > for String { fn partial_cmp (& self , other : & Bytes) -> Option < cmp :: Ordering > { < [u8] as PartialOrd < [u8] > > :: partial_cmp (self . as_bytes () , other) } }
};
}
