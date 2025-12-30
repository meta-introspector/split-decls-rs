// Generated macro for impl_735 (impl)
macro_rules! Depcrate_frame_stream_idimpl_735 {
() => {
// Module: crate::frame::stream_id
// Provides: {"impl_735"}
// Dependencies: {}
impl StreamId { # [doc = " Stream ID 0."] pub const ZERO : StreamId = StreamId (0) ; # [doc = " The maximum allowed stream ID."] pub const MAX : StreamId = StreamId (u32 :: MAX >> 1) ; # [doc = " Parse the stream ID"] # [inline] pub fn parse (buf : & [u8]) -> (StreamId , bool) { let mut ubuf = [0 ; 4] ; ubuf . copy_from_slice (& buf [0 .. 4]) ; let unpacked = u32 :: from_be_bytes (ubuf) ; let flag = unpacked & STREAM_ID_MASK == STREAM_ID_MASK ; (StreamId (unpacked & ! STREAM_ID_MASK) , flag) } # [doc = " Returns true if this stream ID corresponds to a stream that"] # [doc = " was initiated by the client."] pub fn is_client_initiated (& self) -> bool { let id = self . 0 ; id != 0 && id % 2 == 1 } # [doc = " Returns true if this stream ID corresponds to a stream that"] # [doc = " was initiated by the server."] pub fn is_server_initiated (& self) -> bool { let id = self . 0 ; id != 0 && id % 2 == 0 } # [doc = " Return a new `StreamId` for stream 0."] # [inline] pub fn zero () -> StreamId { StreamId :: ZERO } # [doc = " Returns true if this stream ID is zero."] pub fn is_zero (& self) -> bool { self . 0 == 0 } # [doc = " Returns the next stream ID initiated by the same peer as this stream"] # [doc = " ID, or an error if incrementing this stream ID would overflow the"] # [doc = " maximum."] pub fn next_id (& self) -> Result < StreamId , StreamIdOverflow > { let next = self . 0 + 2 ; if next > StreamId :: MAX . 0 { Err (StreamIdOverflow) } else { Ok (StreamId (next)) } } }
};
}
