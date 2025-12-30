// Generated macro for impl_69 (impl)
macro_rules! Depcrate_deserializerimpl_69 {
() => {
// Module: crate::deserializer
// Provides: {"impl_69"}
// Dependencies: {}
impl DeserializeError { # [doc = " Return the field index (starting at 0) of this error, if available."] pub fn field (& self) -> Option < u64 > { self . field } # [doc = " Return the underlying error kind."] pub fn kind (& self) -> & DeserializeErrorKind { & self . kind } }
};
}
