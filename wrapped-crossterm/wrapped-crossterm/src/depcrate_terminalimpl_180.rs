// Generated macro for impl_180 (impl)
macro_rules! Depcrate_terminalimpl_180 {
() => {
// Module: crate::terminal
// Provides: {"impl_180"}
// Dependencies: {}
impl Command for LeaveAlternateScreen { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?1049l")) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { let screen_buffer = ScreenBuffer :: from (Handle :: current_out_handle () ?) ; screen_buffer . show () ? ; Ok (()) } }
};
}
