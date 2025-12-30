// Generated macro for impl_209 (impl)
macro_rules! Depcrate_processimpl_209 {
() => {
// Module: crate::process
// Provides: {"impl_209"}
// Dependencies: {}
impl < T > Termios for & mut T where T : Termios , { fn is_echo (& self) -> Result < bool > { T :: is_echo (self) } fn set_echo (& mut self , on : bool) -> Result < bool > { T :: set_echo (self , on) } }
};
}
