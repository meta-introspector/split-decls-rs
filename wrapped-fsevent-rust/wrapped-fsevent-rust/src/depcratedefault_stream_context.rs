// Generated macro for default_stream_context (function)
macro_rules! Depcratedefault_stream_context {
() => {
// Module: crate
// Provides: {"default_stream_context"}
// Dependencies: {}
fn default_stream_context (event_sender : * const Sender < Event >) -> FSEventStreamContext { let ptr = event_sender as * mut c_void ; FSEventStreamContext { version : 0 , info : ptr , retain : None , release : None , copyDescription : None , } }
};
}
