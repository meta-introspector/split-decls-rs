// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_fsimpl_1242 {
() => {
// Module: crate::fs
// Provides: {"impl_1242"}
// Dependencies: {}
impl Write for File { fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { fd :: write (self . fd , buf) } fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
