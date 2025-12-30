// Generated macro for impl_191 (impl)
macro_rules! Depcrate_terminalimpl_191 {
() => {
// Module: crate::terminal
// Provides: {"impl_191"}
// Dependencies: {}
impl < T : fmt :: Display > Command for SetTitle < T > { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , "\x1B]0;{}\x07" , & self . 0) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { sys :: set_window_title (& self . 0) } }
};
}
