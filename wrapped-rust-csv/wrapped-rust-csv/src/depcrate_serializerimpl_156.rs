// Generated macro for impl_156 (impl)
macro_rules! Depcrate_serializerimpl_156 {
() => {
// Module: crate::serializer
// Provides: {"impl_156"}
// Dependencies: {}
impl SerdeError for Error { fn custom < T : fmt :: Display > (msg : T) -> Error { Error :: new (ErrorKind :: Serialize (msg . to_string ())) } }
};
}
