// Generated macro for impl_178 (impl)
macro_rules! Depcrate_terminalimpl_178 {
() => {
// Module: crate::terminal
// Provides: {"impl_178"}
// Dependencies: {}
impl Command for EnterAlternateScreen { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?1049h")) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { let alternate_screen = ScreenBuffer :: create () ? ; alternate_screen . show () ? ; Ok (()) } }
};
}
