// Generated macro for impl_78 (impl)
macro_rules! Depcrate_eventimpl_78 {
() => {
// Module: crate::event
// Provides: {"impl_78"}
// Dependencies: {}
impl Command for DisableMouseCapture { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (concat ! (csi ! ("?1006l") , csi ! ("?1015l") , csi ! ("?1003l") , csi ! ("?1002l") , csi ! ("?1000l") ,)) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: windows :: disable_mouse_capture () } # [cfg (windows)] fn is_ansi_code_supported (& self) -> bool { false } }
};
}
