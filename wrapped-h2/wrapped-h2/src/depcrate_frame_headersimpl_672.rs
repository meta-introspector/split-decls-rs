// Generated macro for impl_672 (impl)
macro_rules! Depcrate_frame_headersimpl_672 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_672"}
// Dependencies: {}
impl fmt :: Debug for PushPromiseFlag { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { util :: debug_flags (fmt , self . 0) . flag_if (self . is_end_headers () , "END_HEADERS") . flag_if (self . is_padded () , "PADDED") . finish () } }
};
}
