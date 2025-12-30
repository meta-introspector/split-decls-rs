// Generated macro for impl_189 (impl)
macro_rules! Depcrate_terminalimpl_189 {
() => {
// Module: crate::terminal
// Provides: {"impl_189"}
// Dependencies: {}
impl Command for SetSize { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , csi ! ("8;{};{}t") , self . 1 , self . 0) } # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { sys :: set_size (self . 0 , self . 1) } }
};
}
