// Generated macro for impl_752 (impl)
macro_rules! Depcrate_frame_window_updateimpl_752 {
() => {
// Module: crate::frame::window_update
// Provides: {"impl_752"}
// Dependencies: {}
impl WindowUpdate { pub fn new (stream_id : StreamId , size_increment : u32) -> WindowUpdate { WindowUpdate { stream_id , size_increment , } } pub fn stream_id (& self) -> StreamId { self . stream_id } pub fn size_increment (& self) -> u32 { self . size_increment } # [doc = " Builds a `WindowUpdate` frame from a raw frame."] pub fn load (head : Head , payload : & [u8]) -> Result < WindowUpdate , Error > { debug_assert_eq ! (head . kind () , crate :: frame :: Kind :: WindowUpdate) ; if payload . len () != 4 { return Err (Error :: BadFrameSize) ; } let size_increment = unpack_octets_4 ! (payload , 0 , u32) & ! SIZE_INCREMENT_MASK ; if size_increment == 0 { return Err (Error :: InvalidWindowUpdateValue) ; } Ok (WindowUpdate { stream_id : head . stream_id () , size_increment , }) } pub fn encode < B : BufMut > (& self , dst : & mut B) { tracing :: trace ! ("encoding WINDOW_UPDATE; id={:?}" , self . stream_id) ; let head = Head :: new (Kind :: WindowUpdate , 0 , self . stream_id) ; head . encode (4 , dst) ; dst . put_u32 (self . size_increment) ; } }
};
}
