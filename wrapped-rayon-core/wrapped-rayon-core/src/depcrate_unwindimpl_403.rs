// Generated macro for impl_403 (impl)
macro_rules! Depcrate_unwindimpl_403 {
() => {
// Module: crate::unwind
// Provides: {"impl_403"}
// Dependencies: {}
impl Drop for AbortIfPanic { fn drop (& mut self) { eprintln ! ("Rayon: detected unexpected panic; aborting") ; :: std :: process :: abort () ; } }
};
}
