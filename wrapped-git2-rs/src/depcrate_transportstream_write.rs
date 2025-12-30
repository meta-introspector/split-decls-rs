// Generated macro for stream_write (function)
macro_rules! Depcrate_transportstream_write {
() => {
// Module: crate::transport
// Provides: {"stream_write"}
// Dependencies: {}
extern "C" fn stream_write (stream : * mut raw :: git_smart_subtransport_stream , buffer : * const c_char , len : size_t ,) -> c_int { let ret = panic :: wrap (| | unsafe { let transport = & mut * (stream as * mut RawSmartSubtransportStream) ; let buf = slice :: from_raw_parts (buffer as * const u8 , len as usize) ; transport . obj . write_all (buf) }) ; match ret { Some (Ok (())) => 0 , Some (Err (e)) => unsafe { set_err_io (& e) ; - 2 } , None => - 1 , } }
};
}
