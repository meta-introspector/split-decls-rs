// Generated macro for callback (function)
macro_rules! Depcrate_fseventcallback {
() => {
// Module: crate::fsevent
// Provides: {"callback"}
// Dependencies: {}
extern "C" fn callback (stream_ref : fs :: FSEventStreamRef , info : * mut libc :: c_void , num_events : libc :: size_t , event_paths : * mut libc :: c_void , event_flags : * const fs :: FSEventStreamEventFlags , event_ids : * const fs :: FSEventStreamEventId ,) { unsafe { callback_impl (stream_ref , info , num_events , event_paths , event_flags , event_ids ,) } }
};
}
