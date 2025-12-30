// Generated macro for impl_81 (impl)
macro_rules! Depcrate_codecimpl_81 {
() => {
// Module: crate::codec
// Provides: {"impl_81"}
// Dependencies: {}
impl < T , B > Codec < T , B > where T : AsyncWrite + Unpin , B : Buf , { # [doc = " Returns `Ready` when the codec can buffer a frame"] pub fn poll_ready (& mut self , cx : & mut Context) -> Poll < io :: Result < () > > { self . framed_write () . poll_ready (cx) } # [doc = " Buffer a frame."] # [doc = ""] # [doc = " `poll_ready` must be called first to ensure that a frame may be"] # [doc = " accepted."] # [doc = ""] # [doc = " TODO: Rename this to avoid conflicts with Sink::buffer"] pub fn buffer (& mut self , item : Frame < B >) -> Result < () , UserError > { self . framed_write () . buffer (item) } # [doc = " Flush buffered data to the wire"] pub fn flush (& mut self , cx : & mut Context) -> Poll < io :: Result < () > > { self . framed_write () . flush (cx) } # [doc = " Shutdown the send half"] pub fn shutdown (& mut self , cx : & mut Context) -> Poll < io :: Result < () > > { self . framed_write () . shutdown (cx) } }
};
}
