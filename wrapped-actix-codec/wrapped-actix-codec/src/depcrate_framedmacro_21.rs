// Generated macro for macro_21 (macro)
macro_rules! Depcrate_framedmacro_21 {
() => {
// Module: crate::framed
// Provides: {"macro_21"}
// Dependencies: {}
pin_project ! { # [doc = " A unified `Stream` and `Sink` interface to an underlying I/O object, using the `Encoder` and"] # [doc = " `Decoder` traits to encode and decode frames."] # [doc = ""] # [doc = " Raw I/O objects work with byte sequences, but higher-level code usually wants to batch these"] # [doc = " into meaningful chunks, called \"frames\". This method layers framing on top of an I/O object,"] # [doc = " by using the `Encoder`/`Decoder` traits to handle encoding and decoding of message frames."] # [doc = " Note that the incoming and outgoing frame types may be distinct."] pub struct Framed < T , U > { # [pin] io : T , codec : U , flags : Flags , read_buf : BytesMut , write_buf : BytesMut , } }
};
}
