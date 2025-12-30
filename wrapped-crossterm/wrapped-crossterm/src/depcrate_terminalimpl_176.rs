// Generated macro for impl_176 (impl)
macro_rules! Depcrate_terminalimpl_176 {
() => {
// Module: crate::terminal
// Provides: {"impl_176"}
// Dependencies: {}
impl Command for EnableLineWrap { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { f . write_str (csi ! ("?7h")) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { let screen_buffer = ScreenBuffer :: current () ? ; let console_mode = ConsoleMode :: from (screen_buffer . handle () . clone ()) ; let new_mode = console_mode . mode () ? | ENABLE_WRAP_AT_EOL_OUTPUT ; console_mode . set_mode (new_mode) ? ; Ok (()) } }
};
}
