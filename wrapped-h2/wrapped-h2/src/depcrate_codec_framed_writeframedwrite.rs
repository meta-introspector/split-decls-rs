// Generated macro for FramedWrite (struct)
macro_rules! Depcrate_codec_framed_writeFramedWrite {
() => {
// Module: crate::codec::framed_write
// Provides: {"FramedWrite"}
// Dependencies: {}
# [derive (Debug)] pub struct FramedWrite < T , B > { # [doc = " Upstream `AsyncWrite`"] inner : T , final_flush_done : bool , encoder : Encoder < B > , }
};
}
