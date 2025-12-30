// Generated macro for PushPromise (struct)
macro_rules! Depcrate_frame_headersPushPromise {
() => {
// Module: crate::frame::headers
// Provides: {"PushPromise"}
// Dependencies: {}
# [derive (Eq , PartialEq)] pub struct PushPromise { # [doc = " The ID of the stream with which this frame is associated."] stream_id : StreamId , # [doc = " The ID of the stream being reserved by this PushPromise."] promised_id : StreamId , # [doc = " The header block fragment"] header_block : HeaderBlock , # [doc = " The associated flags"] flags : PushPromiseFlag , }
};
}
