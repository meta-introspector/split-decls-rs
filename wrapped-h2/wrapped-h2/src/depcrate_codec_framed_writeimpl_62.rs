// Generated macro for impl_62 (impl)
macro_rules! Depcrate_codec_framed_writeimpl_62 {
() => {
// Module: crate::codec::framed_write
// Provides: {"impl_62"}
// Dependencies: {}
impl < T : AsyncRead + Unpin , B > AsyncRead for FramedWrite < T , B > { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf ,) -> Poll < io :: Result < () > > { Pin :: new (& mut self . inner) . poll_read (cx , buf) } }
};
}
