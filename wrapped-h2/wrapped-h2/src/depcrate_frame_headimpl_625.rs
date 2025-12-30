// Generated macro for impl_625 (impl)
macro_rules! Depcrate_frame_headimpl_625 {
() => {
// Module: crate::frame::head
// Provides: {"impl_625"}
// Dependencies: {}
impl Head { pub fn new (kind : Kind , flag : u8 , stream_id : StreamId) -> Head { Head { kind , flag , stream_id , } } # [doc = " Parse an HTTP/2 frame header"] pub fn parse (header : & [u8]) -> Head { let (stream_id , _) = StreamId :: parse (& header [5 ..]) ; Head { kind : Kind :: new (header [3]) , flag : header [4] , stream_id , } } pub fn stream_id (& self) -> StreamId { self . stream_id } pub fn kind (& self) -> Kind { self . kind } pub fn flag (& self) -> u8 { self . flag } pub fn encode_len (& self) -> usize { super :: HEADER_LEN } pub fn encode < T : BufMut > (& self , payload_len : usize , dst : & mut T) { debug_assert ! (self . encode_len () <= dst . remaining_mut ()) ; dst . put_uint (payload_len as u64 , 3) ; dst . put_u8 (self . kind as u8) ; dst . put_u8 (self . flag) ; dst . put_u32 (self . stream_id . into ()) ; } }
};
}
