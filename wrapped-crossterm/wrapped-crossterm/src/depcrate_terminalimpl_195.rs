// Generated macro for impl_195 (impl)
macro_rules! Depcrate_terminalimpl_195 {
() => {
// Module: crate::terminal
// Provides: {"impl_195"}
// Dependencies: {}
impl Command for EndSynchronizedUpdate { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?2026l")) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { Ok (()) } # [cfg (windows)] # [inline] fn is_ansi_code_supported (& self) -> bool { true } }
};
}
