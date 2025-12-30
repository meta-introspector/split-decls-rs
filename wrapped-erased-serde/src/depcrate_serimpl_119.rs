// Generated macro for impl_119 (impl)
macro_rules! Depcrate_serimpl_119 {
() => {
// Module: crate::ser
// Provides: {"impl_119"}
// Dependencies: {}
impl serde :: ser :: Error for ErrorImpl { fn custom < T : Display > (msg : T) -> Self { ErrorImpl :: Custom (Box :: new (msg . to_string ())) } }
};
}
