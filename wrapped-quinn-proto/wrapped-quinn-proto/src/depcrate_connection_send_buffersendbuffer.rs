// Generated macro for SendBuffer (struct)
macro_rules! Depcrate_connection_send_bufferSendBuffer {
() => {
// Module: crate::connection::send_buffer
// Provides: {"SendBuffer"}
// Dependencies: {}
# [doc = " Buffer of outgoing retransmittable stream data"] # [derive (Default , Debug)] pub (super) struct SendBuffer { # [doc = " Data queued by the application but not yet acknowledged. May or may not have been sent."] unacked_segments : VecDeque < Bytes > , # [doc = " Total size of `unacked_segments`"] unacked_len : usize , # [doc = " The first offset that hasn't been written by the application, i.e. the offset past the end of `unacked`"] offset : u64 , # [doc = " The first offset that hasn't been sent"] # [doc = ""] # [doc = " Always lies in (offset - unacked.len())..offset"] unsent : u64 , # [doc = " Acknowledged ranges which couldn't be discarded yet as they don't include the earliest"] # [doc = " offset in `unacked`"] acks : RangeSet , # [doc = " Previously transmitted ranges deemed lost"] retransmits : RangeSet , }
};
}
