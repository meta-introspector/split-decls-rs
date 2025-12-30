// Generated macro for impl_83 (impl)
macro_rules! Depcrate_errorimpl_83 {
() => {
// Module: crate::error
// Provides: {"impl_83"}
// Dependencies: {}
impl serde :: ser :: Error for Error { fn custom < T : Display > (msg : T) -> Self { let imp = Box :: new (ErrorImpl :: Custom (msg . to_string ())) ; Error { imp } } }
};
}
