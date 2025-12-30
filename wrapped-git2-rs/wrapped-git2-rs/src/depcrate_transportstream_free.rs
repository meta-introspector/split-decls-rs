// Generated macro for stream_free (function)
macro_rules! Depcrate_transportstream_free {
() => {
// Module: crate::transport
// Provides: {"stream_free"}
// Dependencies: {}
extern "C" fn stream_free (stream : * mut raw :: git_smart_subtransport_stream) { let _ = panic :: wrap (| | unsafe { mem :: transmute :: < _ , Box < RawSmartSubtransportStream > > (stream) ; }) ; }
};
}
