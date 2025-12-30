// Generated macro for impl_2316 (impl)
macro_rules! Depcrate_io_into_sinkimpl_2316 {
() => {
// Module: crate::io::into_sink
// Provides: {"impl_2316"}
// Dependencies: {}
impl < W : AsyncWrite , Item : AsRef < [u8] > > IntoSink < W , Item > { pub (super) fn new (writer : W) -> Self { Self { writer , buffer : None } } # [doc = " If we have an outstanding block in `buffer` attempt to push it into the writer, does _not_"] # [doc = " flush the writer after it succeeds in pushing the block into it."] fn poll_flush_buffer (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , io :: Error > > { let mut this = self . project () ; if let Some (buffer) = this . buffer { loop { let bytes = buffer . bytes . as_ref () ; let written = ready ! (this . writer . as_mut () . poll_write (cx , & bytes [buffer . offset ..])) ? ; buffer . offset += written ; if buffer . offset == bytes . len () { break ; } } } * this . buffer = None ; Poll :: Ready (Ok (())) } }
};
}
