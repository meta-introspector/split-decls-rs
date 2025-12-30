// Generated macro for impl_660 (impl)
macro_rules! Depcrate_frame_headersimpl_660 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_660"}
// Dependencies: {}
impl fmt :: Debug for PushPromise { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("PushPromise") . field ("stream_id" , & self . stream_id) . field ("promised_id" , & self . promised_id) . field ("flags" , & self . flags) . finish () } }
};
}
