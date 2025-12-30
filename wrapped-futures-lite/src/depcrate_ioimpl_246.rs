// Generated macro for impl_246 (impl)
macro_rules! Depcrate_ioimpl_246 {
() => {
// Module: crate::io
// Provides: {"impl_246"}
// Dependencies: {}
impl < T : AsyncWrite + Unpin > std :: io :: Write for AsyncAsSync < '_ , '_ , T > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize > { self . poll_with (| io , cx | io . poll_write (cx , buf)) } # [inline] fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> Result < usize > { self . poll_with (| io , cx | io . poll_write_vectored (cx , bufs)) } # [inline] fn flush (& mut self) -> Result < () > { self . poll_with (| io , cx | io . poll_flush (cx)) } }
};
}
