// Generated macro for impl_736 (impl)
macro_rules! Depcrate_frame_stream_idimpl_736 {
() => {
// Module: crate::frame::stream_id
// Provides: {"impl_736"}
// Dependencies: {}
impl From < u32 > for StreamId { fn from (src : u32) -> Self { assert_eq ! (src & STREAM_ID_MASK , 0 , "invalid stream ID -- MSB is set") ; StreamId (src) } }
};
}
