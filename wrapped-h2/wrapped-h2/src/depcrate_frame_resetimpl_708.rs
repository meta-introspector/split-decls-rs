// Generated macro for impl_708 (impl)
macro_rules! Depcrate_frame_resetimpl_708 {
() => {
// Module: crate::frame::reset
// Provides: {"impl_708"}
// Dependencies: {}
impl Reset { pub fn new (stream_id : StreamId , error : Reason) -> Reset { Reset { stream_id , error_code : error , } } pub fn stream_id (& self) -> StreamId { self . stream_id } pub fn reason (& self) -> Reason { self . error_code } pub fn load (head : Head , payload : & [u8]) -> Result < Reset , Error > { if payload . len () != 4 { return Err (Error :: InvalidPayloadLength) ; } let error_code = unpack_octets_4 ! (payload , 0 , u32) ; Ok (Reset { stream_id : head . stream_id () , error_code : error_code . into () , }) } pub fn encode < B : BufMut > (& self , dst : & mut B) { tracing :: trace ! ("encoding RESET; id={:?} code={:?}" , self . stream_id , self . error_code) ; let head = Head :: new (Kind :: Reset , 0 , self . stream_id) ; head . encode (4 , dst) ; dst . put_u32 (self . error_code . into ()) ; } }
};
}
