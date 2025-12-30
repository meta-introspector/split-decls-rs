// Generated macro for impl_75 (impl)
macro_rules! Depcrate_driver_applyimpl_75 {
() => {
// Module: crate::driver::apply
// Provides: {"impl_75"}
// Dependencies: {}
impl Drop for WriterThread { fn drop (& mut self) { if let Err (_err) = self . join () { gix_trace :: debug ! (err = % _err , "Failed to join writer thread during drop") ; } } }
};
}
