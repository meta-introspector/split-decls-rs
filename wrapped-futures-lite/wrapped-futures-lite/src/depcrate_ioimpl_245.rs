// Generated macro for impl_245 (impl)
macro_rules! Depcrate_ioimpl_245 {
() => {
// Module: crate::io
// Provides: {"impl_245"}
// Dependencies: {}
impl < T : AsyncRead + Unpin > std :: io :: Read for AsyncAsSync < '_ , '_ , T > { # [inline] fn read (& mut self , buf : & mut [u8]) -> Result < usize > { self . poll_with (| io , cx | io . poll_read (cx , buf)) } # [inline] fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> Result < usize > { self . poll_with (| io , cx | io . poll_read_vectored (cx , bufs)) } }
};
}
