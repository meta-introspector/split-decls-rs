// Generated macro for impl_140 (impl)
macro_rules! Depcrate_bytesimpl_140 {
() => {
// Module: crate::bytes
// Provides: {"impl_140"}
// Dependencies: {}
impl PartialOrd < str > for Bytes { fn partial_cmp (& self , other : & str) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_bytes ()) } }
};
}
