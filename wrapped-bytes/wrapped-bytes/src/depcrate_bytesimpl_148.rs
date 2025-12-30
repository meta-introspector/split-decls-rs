// Generated macro for impl_148 (impl)
macro_rules! Depcrate_bytesimpl_148 {
() => {
// Module: crate::bytes
// Provides: {"impl_148"}
// Dependencies: {}
impl PartialOrd < String > for Bytes { fn partial_cmp (& self , other : & String) -> Option < cmp :: Ordering > { self . as_slice () . partial_cmp (other . as_bytes ()) } }
};
}
