// Generated macro for impl_75 (impl)
macro_rules! Depcrate_consoleimpl_75 {
() => {
// Module: crate::console
// Provides: {"impl_75"}
// Dependencies: {}
impl Read for Console { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { self . device . read (buf) } }
};
}
