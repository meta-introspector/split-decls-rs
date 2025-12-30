// Generated macro for FramedParts (struct)
macro_rules! Depcrate_framedFramedParts {
() => {
// Module: crate::framed
// Provides: {"FramedParts"}
// Dependencies: {}
# [doc = " `FramedParts` contains an export of the data of a Framed transport."] # [doc = ""] # [doc = " It can be used to construct a new `Framed` with a different codec. It contains all current"] # [doc = " buffers and the inner transport."] # [derive (Debug)] pub struct FramedParts < T , U > { # [doc = " The inner transport used to read bytes to and write bytes to."] pub io : T , # [doc = " The codec object."] pub codec : U , # [doc = " The buffer with read but unprocessed data."] pub read_buf : BytesMut , # [doc = " A buffer with unprocessed data which are not written yet."] pub write_buf : BytesMut , flags : Flags , }
};
}
