// Generated macro for impl_74 (impl)
macro_rules! Depcrate_client_blocking_io_fileimpl_74 {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"impl_74"}
// Dependencies: {}
impl Drop for SpawnProcessOnDemand { fn drop (& mut self) { if let Some (mut child) = self . child . take () { child . kill () . ok () ; child . wait () . ok () ; } } }
};
}
