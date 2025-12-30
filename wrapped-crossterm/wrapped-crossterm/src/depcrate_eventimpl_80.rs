// Generated macro for impl_80 (impl)
macro_rules! Depcrate_eventimpl_80 {
() => {
// Module: crate::event
// Provides: {"impl_80"}
// Dependencies: {}
impl Command for EnableFocusChange { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?1004h")) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { Ok (()) } }
};
}
