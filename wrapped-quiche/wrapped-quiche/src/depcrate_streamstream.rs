// Generated macro for Stream (struct)
macro_rules! Depcrate_streamStream {
() => {
// Module: crate::stream
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " A QUIC stream."] pub struct Stream < F : BufFactory = DefaultBufFactory > { # [doc = " Receive-side stream buffer."] pub recv : recv_buf :: RecvBuf , # [doc = " Send-side stream buffer."] pub send : send_buf :: SendBuf < F > , pub send_lowat : usize , # [doc = " Whether the stream is bidirectional."] pub bidi : bool , # [doc = " Whether the stream was created by the local endpoint."] pub local : bool , # [doc = " The stream's urgency (lower is better). Default is `DEFAULT_URGENCY`."] pub urgency : u8 , # [doc = " Whether the stream can be flushed incrementally. Default is `true`."] pub incremental : bool , pub priority_key : Arc < StreamPriorityKey > , }
};
}
