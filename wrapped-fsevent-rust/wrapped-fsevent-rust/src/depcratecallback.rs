// Generated macro for callback (function)
macro_rules! Depcratecallback {
() => {
// Module: crate
// Provides: {"callback"}
// Dependencies: {}
unsafe extern "C-unwind" fn callback (_stream_ref : ConstFSEventStreamRef , info : * mut c_void , num_events : usize , event_paths : NonNull < c_void > , event_flags : NonNull < FSEventStreamEventFlags > , event_ids : NonNull < FSEventStreamEventId > ,) { let event_paths = unsafe { slice :: from_raw_parts (event_paths . as_ptr () as * const * const i8 , num_events) } ; let event_flags = unsafe { slice :: from_raw_parts (event_flags . as_ptr () , num_events) } ; let event_ids = unsafe { slice :: from_raw_parts (event_ids . as_ptr () , num_events) } ; let sender = unsafe { (info as * mut Sender < Event >) . as_mut () . expect ("Invalid Sender<Event>.") } ; for event in event_paths . iter () . zip (event_flags) . zip (event_ids) . map (| ((& path , & flag) , & id) | unsafe { let path = CStr :: from_ptr (path) . to_str () . expect ("Invalid UTF8 string.") ; Event { event_id : id , flag : StreamFlags :: from_bits (flag) . unwrap_or_else (| | { panic ! ("Unable to decode StreamFlags: {} for {}" , flag , path) }) , path : path . to_string () , } }) { let _s = sender . send (event) ; } }
};
}
