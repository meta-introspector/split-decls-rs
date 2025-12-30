// Generated macro for impl_185 (impl)
macro_rules! Depcrate_terminalimpl_185 {
() => {
// Module: crate::terminal
// Provides: {"impl_185"}
// Dependencies: {}
impl Command for ScrollDown { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { if self . 0 != 0 { write ! (f , csi ! ("{}T") , self . 0) ? ; } Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { sys :: scroll_down (self . 0) } }
};
}
