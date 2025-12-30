// Generated macro for impl_158 (impl)
macro_rules! Depcrate_process_uniximpl_158 {
() => {
// Module: crate::process::unix
// Provides: {"impl_158"}
// Dependencies: {}
impl Termios for UnixProcess { fn is_echo (& self) -> Result < bool > { let value = self . proc . get_echo () ? ; Ok (value) } fn set_echo (& mut self , on : bool) -> Result < bool > { let value = self . proc . set_echo (on , None) ? ; Ok (value) } }
};
}
