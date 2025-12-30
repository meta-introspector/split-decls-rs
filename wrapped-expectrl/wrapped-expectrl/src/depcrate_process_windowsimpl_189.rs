// Generated macro for impl_189 (impl)
macro_rules! Depcrate_process_windowsimpl_189 {
() => {
// Module: crate::process::windows
// Provides: {"impl_189"}
// Dependencies: {}
impl Healthcheck for WinProcess { type Status = () ; fn get_status (& self) -> Result < Self :: Status > { Ok (()) } fn is_alive (& self) -> Result < bool > { Ok (self . proc . is_alive ()) } }
};
}
