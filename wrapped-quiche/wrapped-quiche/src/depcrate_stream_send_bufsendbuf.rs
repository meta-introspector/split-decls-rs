// Generated macro for SendBuf (struct)
macro_rules! Depcrate_stream_send_bufSendBuf {
() => {
// Module: crate::stream::send_buf
// Provides: {"SendBuf"}
// Dependencies: {}
# [doc = " Send-side stream buffer."] # [doc = ""] # [doc = " Stream data scheduled to be sent to the peer is buffered in a list of data"] # [doc = " chunks ordered by offset in ascending order. Contiguous data can then be"] # [doc = " read into a slice."] # [doc = ""] # [doc = " By default, new data is appended at the end of the stream, but data can be"] # [doc = " inserted at the start of the buffer (this is to allow data that needs to be"] # [doc = " retransmitted to be re-buffered)."] # [derive (Debug , Default)] pub struct SendBuf < F = DefaultBufFactory > where F : BufFactory , { # [doc = " Chunks of data to be sent, ordered by offset."] data : VecDeque < RangeBuf < F > > , # [doc = " The index of the buffer that needs to be sent next."] pos : usize , # [doc = " The maximum offset of data buffered in the stream."] off : u64 , # [doc = " The maximum offset of data sent to the peer, regardless of"] # [doc = " retransmissions."] emit_off : u64 , # [doc = " The amount of data currently buffered."] len : u64 , # [doc = " The maximum offset we are allowed to send to the peer."] max_data : u64 , # [doc = " The last offset the stream was blocked at, if any."] blocked_at : Option < u64 > , # [doc = " The final stream offset written to the stream, if any."] fin_off : Option < u64 > , # [doc = " Whether the stream's send-side has been shut down."] shutdown : bool , # [doc = " Ranges of data offsets that have been acked."] acked : ranges :: RangeSet , # [doc = " The error code received via STOP_SENDING."] error : Option < u64 > , }
};
}
