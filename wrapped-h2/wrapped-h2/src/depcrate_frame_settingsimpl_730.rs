// Generated macro for impl_730 (impl)
macro_rules! Depcrate_frame_settingsimpl_730 {
() => {
// Module: crate::frame::settings
// Provides: {"impl_730"}
// Dependencies: {}
impl fmt :: Debug for SettingsFlags { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { util :: debug_flags (f , self . 0) . flag_if (self . is_ack () , "ACK") . finish () } }
};
}
