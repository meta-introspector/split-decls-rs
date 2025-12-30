// Generated macro for impl_611 (impl)
macro_rules! Depcrate_frame_dataimpl_611 {
() => {
// Module: crate::frame::data
// Provides: {"impl_611"}
// Dependencies: {}
impl fmt :: Debug for DataFlags { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { util :: debug_flags (fmt , self . 0) . flag_if (self . is_end_stream () , "END_STREAM") . flag_if (self . is_padded () , "PADDED") . finish () } }
};
}
