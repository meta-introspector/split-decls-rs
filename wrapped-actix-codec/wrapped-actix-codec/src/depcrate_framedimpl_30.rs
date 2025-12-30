// Generated macro for impl_30 (impl)
macro_rules! Depcrate_framedimpl_30 {
() => {
// Module: crate::framed
// Provides: {"impl_30"}
// Dependencies: {}
impl < T , U > FramedParts < T , U > { # [doc = " Creates a new default `FramedParts`."] pub fn new (io : T , codec : U) -> FramedParts < T , U > { FramedParts { io , codec , flags : Flags :: empty () , read_buf : BytesMut :: new () , write_buf : BytesMut :: new () , } } # [doc = " Creates a new `FramedParts` with read buffer."] pub fn with_read_buf (io : T , codec : U , read_buf : BytesMut) -> FramedParts < T , U > { FramedParts { io , codec , read_buf , flags : Flags :: empty () , write_buf : BytesMut :: new () , } } }
};
}
