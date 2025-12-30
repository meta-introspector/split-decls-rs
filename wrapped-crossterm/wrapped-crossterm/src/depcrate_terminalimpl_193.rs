// Generated macro for impl_193 (impl)
macro_rules! Depcrate_terminalimpl_193 {
() => {
// Module: crate::terminal
// Provides: {"impl_193"}
// Dependencies: {}
impl Command for BeginSynchronizedUpdate { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?2026h")) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { Ok (()) } # [cfg (windows)] # [inline] fn is_ansi_code_supported (& self) -> bool { true } }
};
}
