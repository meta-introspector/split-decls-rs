// Generated macro for impl_617 (impl)
macro_rules! Depcrate_frame_go_awayimpl_617 {
() => {
// Module: crate::frame::go_away
// Provides: {"impl_617"}
// Dependencies: {}
impl GoAway { pub fn new (last_stream_id : StreamId , reason : Reason) -> Self { GoAway { last_stream_id , error_code : reason , debug_data : Bytes :: new () , } } pub fn with_debug_data (last_stream_id : StreamId , reason : Reason , debug_data : Bytes) -> Self { Self { last_stream_id , error_code : reason , debug_data , } } pub fn last_stream_id (& self) -> StreamId { self . last_stream_id } pub fn reason (& self) -> Reason { self . error_code } pub fn debug_data (& self) -> & Bytes { & self . debug_data } pub fn load (payload : & [u8]) -> Result < GoAway , Error > { if payload . len () < 8 { return Err (Error :: BadFrameSize) ; } let (last_stream_id , _) = StreamId :: parse (& payload [.. 4]) ; let error_code = unpack_octets_4 ! (payload , 4 , u32) ; let debug_data = Bytes :: copy_from_slice (& payload [8 ..]) ; Ok (GoAway { last_stream_id , error_code : error_code . into () , debug_data , }) } pub fn encode < B : BufMut > (& self , dst : & mut B) { tracing :: trace ! ("encoding GO_AWAY; code={:?}" , self . error_code) ; let head = Head :: new (Kind :: GoAway , 0 , StreamId :: zero ()) ; head . encode (8 + self . debug_data . len () , dst) ; dst . put_u32 (self . last_stream_id . into ()) ; dst . put_u32 (self . error_code . into ()) ; dst . put (self . debug_data . slice (..)) ; } }
};
}
