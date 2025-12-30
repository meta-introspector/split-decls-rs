// Generated macro for impl_608 (impl)
macro_rules! Depcrate_frame_dataimpl_608 {
() => {
// Module: crate::frame::data
// Provides: {"impl_608"}
// Dependencies: {}
impl < T > fmt :: Debug for Data < T > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut f = fmt . debug_struct ("Data") ; f . field ("stream_id" , & self . stream_id) ; if ! self . flags . is_empty () { f . field ("flags" , & self . flags) ; } if let Some (ref pad_len) = self . pad_len { f . field ("pad_len" , pad_len) ; } f . finish () } }
};
}
