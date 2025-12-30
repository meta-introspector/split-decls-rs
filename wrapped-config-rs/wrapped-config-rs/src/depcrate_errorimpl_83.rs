// Generated macro for impl_83 (impl)
macro_rules! Depcrate_errorimpl_83 {
() => {
// Module: crate::error
// Provides: {"impl_83"}
// Dependencies: {}
impl de :: Error for ConfigError { fn custom < T : fmt :: Display > (msg : T) -> Self { Self :: Message (msg . to_string ()) } fn missing_field (field : & 'static str) -> Self { Self :: NotFound (field . into ()) } }
};
}
