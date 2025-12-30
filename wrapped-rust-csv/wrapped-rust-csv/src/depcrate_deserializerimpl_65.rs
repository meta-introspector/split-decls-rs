// Generated macro for impl_65 (impl)
macro_rules! Depcrate_deserializerimpl_65 {
() => {
// Module: crate::deserializer
// Provides: {"impl_65"}
// Dependencies: {}
impl SerdeError for DeserializeError { fn custom < T : fmt :: Display > (msg : T) -> DeserializeError { DeserializeError { field : None , kind : DEK :: Message (msg . to_string ()) } } }
};
}
