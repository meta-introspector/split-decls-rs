// Generated macro for impl_654 (impl)
macro_rules! Depcrate_frame_headersimpl_654 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_654"}
// Dependencies: {}
impl fmt :: Debug for Headers { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut builder = f . debug_struct ("Headers") ; builder . field ("stream_id" , & self . stream_id) . field ("flags" , & self . flags) ; if let Some (ref protocol) = self . header_block . pseudo . protocol { builder . field ("protocol" , protocol) ; } if let Some (ref dep) = self . stream_dep { builder . field ("stream_dep" , dep) ; } builder . finish () } }
};
}
