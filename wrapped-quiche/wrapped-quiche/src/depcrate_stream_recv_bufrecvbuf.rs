// Generated macro for RecvBuf (struct)
macro_rules! Depcrate_stream_recv_bufRecvBuf {
() => {
// Module: crate::stream::recv_buf
// Provides: {"RecvBuf"}
// Dependencies: {}
# [doc = " Receive-side stream buffer."] # [doc = ""] # [doc = " Stream data received by the peer is buffered in a list of data chunks"] # [doc = " ordered by offset in ascending order. Contiguous data can then be read"] # [doc = " into a slice."] # [derive (Debug , Default)] pub struct RecvBuf { # [doc = " Chunks of data received from the peer that have not yet been read by"] # [doc = " the application, ordered by offset."] data : BTreeMap < u64 , RangeBuf > , # [doc = " The lowest data offset that has yet to be read by the application."] off : u64 , # [doc = " The total length of data received on this stream."] len : u64 , # [doc = " Receiver flow controller."] flow_control : flowcontrol :: FlowControl , # [doc = " The final stream offset received from the peer, if any."] fin_off : Option < u64 > , # [doc = " The error code received via RESET_STREAM."] error : Option < u64 > , # [doc = " Whether incoming data is validated but not buffered."] drain : bool , }
};
}
