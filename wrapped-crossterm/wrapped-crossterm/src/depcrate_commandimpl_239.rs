// Generated macro for impl_239 (impl)
macro_rules! Depcrate_commandimpl_239 {
() => {
// Module: crate::command
// Provides: {"impl_239"}
// Dependencies: {}
impl < T : Command + ? Sized > Command for & T { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { (* * self) . write_ansi (f) } # [inline] # [cfg (windows)] fn execute_winapi (& self) -> io :: Result < () > { T :: execute_winapi (self) } # [cfg (windows)] # [inline] fn is_ansi_code_supported (& self) -> bool { T :: is_ansi_code_supported (self) } }
};
}
