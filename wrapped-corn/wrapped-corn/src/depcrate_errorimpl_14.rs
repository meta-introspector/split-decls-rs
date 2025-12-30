// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl serde :: de :: Error for Error { fn custom < T > (msg : T) -> Self where T : Display , { Error :: DeserializationError (msg . to_string ()) } }
};
}
