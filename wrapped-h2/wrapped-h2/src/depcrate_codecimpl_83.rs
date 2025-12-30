// Generated macro for impl_83 (impl)
macro_rules! Depcrate_codecimpl_83 {
() => {
// Module: crate::codec
// Provides: {"impl_83"}
// Dependencies: {}
impl < T , B > Sink < Frame < B > > for Codec < T , B > where T : AsyncWrite + Unpin , B : Buf , { type Error = SendError ; fn start_send (mut self : Pin < & mut Self > , item : Frame < B >) -> Result < () , Self :: Error > { Codec :: buffer (& mut self , item) ? ; Ok (()) } # [doc = " Returns `Ready` when the codec can buffer a frame"] fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . framed_write () . poll_ready (cx) . map_err (Into :: into) } # [doc = " Flush buffered data to the wire"] fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . framed_write () . flush (cx) . map_err (Into :: into) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { ready ! (self . shutdown (cx)) ? ; Poll :: Ready (Ok (())) } }
};
}
