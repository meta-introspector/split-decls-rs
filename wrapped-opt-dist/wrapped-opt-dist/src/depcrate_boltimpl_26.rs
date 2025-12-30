// Generated macro for impl_26 (impl)
macro_rules! Depcrate_boltimpl_26 {
() => {
// Module: crate::bolt
// Provides: {"impl_26"}
// Dependencies: {}
impl Drop for BackedUpFile { fn drop (& mut self) { copy_file (& self . backup , & self . original) . expect ("Cannot restore backed up file") ; } }
};
}
