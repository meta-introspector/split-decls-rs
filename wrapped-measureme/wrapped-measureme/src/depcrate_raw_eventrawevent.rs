// Generated macro for RawEvent (struct)
macro_rules! Depcrate_raw_eventRawEvent {
() => {
// Module: crate::raw_event
// Provides: {"RawEvent"}
// Dependencies: {}
# [doc = " `RawEvent` is how events are stored on-disk. If you change this struct,"] # [doc = " make sure that you increment `file_header::CURRENT_FILE_FORMAT_VERSION`."] # [derive (Eq , PartialEq , Debug)] # [repr (C)] pub struct RawEvent { pub event_kind : StringId , pub event_id : EventId , pub thread_id : u32 , pub payload1_lower : u32 , pub payload2_lower : u32 , pub payloads_upper : u32 , }
};
}
