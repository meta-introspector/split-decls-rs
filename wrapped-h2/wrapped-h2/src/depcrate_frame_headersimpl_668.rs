// Generated macro for impl_668 (impl)
macro_rules! Depcrate_frame_headersimpl_668 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_668"}
// Dependencies: {}
impl fmt :: Debug for HeadersFlag { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { util :: debug_flags (fmt , self . 0) . flag_if (self . is_end_headers () , "END_HEADERS") . flag_if (self . is_end_stream () , "END_STREAM") . flag_if (self . is_padded () , "PADDED") . flag_if (self . is_priority () , "PRIORITY") . finish () } }
};
}
