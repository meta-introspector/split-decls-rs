// Generated macro for impl_47 (impl)
macro_rules! Depcrate_ser_errorimpl_47 {
() => {
// Module: crate::ser::error
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : Debug > SerError for Error < T > { fn custom < U : Display > (msg : U) -> Self { Error :: Value (msg . to_string ()) } }
};
}
