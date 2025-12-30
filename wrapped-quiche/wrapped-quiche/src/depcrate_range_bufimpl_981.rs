// Generated macro for impl_981 (impl)
macro_rules! Depcrate_range_bufimpl_981 {
() => {
// Module: crate::range_buf
// Provides: {"impl_981"}
// Dependencies: {}
impl BufFactory for DefaultBufFactory { type Buf = DefaultBuf ; fn buf_from_slice (buf : & [u8]) -> Self :: Buf { DefaultBuf (Arc :: new (buf . into ())) } }
};
}
