// Generated macro for impl_28 (impl)
macro_rules! Depcrate_framedimpl_28 {
() => {
// Module: crate::framed
// Provides: {"impl_28"}
// Dependencies: {}
impl < T , U > Framed < T , U > { # [doc = " This function returns a *single* object that is both `Stream` and `Sink`; grouping this into"] # [doc = " a single object is often useful for layering things like gzip or TLS, which require both"] # [doc = " read and write access to the underlying object."] # [doc = ""] # [doc = " These objects take a stream, a read buffer and a write buffer. These fields can be obtained"] # [doc = " from an existing `Framed` with the `into_parts` method."] pub fn from_parts (parts : FramedParts < T , U >) -> Framed < T , U > { Framed { io : parts . io , codec : parts . codec , flags : parts . flags , write_buf : parts . write_buf , read_buf : parts . read_buf , } } # [doc = " Consumes the `Frame`, returning its underlying I/O stream, the buffer with unprocessed data,"] # [doc = " and the codec."] # [doc = ""] # [doc = " Note that care should be taken to not tamper with the underlying stream of data coming in as"] # [doc = " it may corrupt the stream of frames otherwise being worked with."] pub fn into_parts (self) -> FramedParts < T , U > { FramedParts { io : self . io , codec : self . codec , flags : self . flags , read_buf : self . read_buf , write_buf : self . write_buf , } } }
};
}
