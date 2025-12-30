// Generated macro for impl_183 (impl)
macro_rules! Depcrate_terminalimpl_183 {
() => {
// Module: crate::terminal
// Provides: {"impl_183"}
// Dependencies: {}
impl Command for ScrollUp { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { if self . 0 != 0 { write ! (f , csi ! ("{}S") , self . 0) ? ; } Ok (()) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { sys :: scroll_up (self . 0) } }
};
}
