// Generated macro for impl_328 (impl)
macro_rules! Depcrate_errorimpl_328 {
() => {
// Module: crate::error
// Provides: {"impl_328"}
// Dependencies: {}
impl Drop for Accumulator { fn drop (& mut self) { if ! std :: thread :: panicking () { if let Some (errors) = & mut self . 0 { match errors . len () { 0 => panic ! ("darling::error::Accumulator dropped without being finished") , error_count => panic ! ("darling::error::Accumulator dropped without being finished. {} errors were lost." , error_count) } } } } }
};
}
