// Generated macro for impl_76 (impl)
macro_rules! Depcrate_eventimpl_76 {
() => {
// Module: crate::event
// Provides: {"impl_76"}
// Dependencies: {}
# [cfg (feature = "events")] impl Command for EnableMouseCapture { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (concat ! (csi ! ("?1000h") , csi ! ("?1002h") , csi ! ("?1003h") , csi ! ("?1015h") , csi ! ("?1006h") ,)) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { sys :: windows :: enable_mouse_capture () } # [cfg (windows)] fn is_ansi_code_supported (& self) -> bool { false } }
};
}
