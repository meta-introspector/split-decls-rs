// Generated macro for impl_108 (impl)
macro_rules! Depcrate_connectionimpl_108 {
() => {
// Module: crate::connection
// Provides: {"impl_108"}
// Dependencies: {}
impl std :: error :: Error for MessageError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { MessageError :: Deserialization (err) => Some (err) , MessageError :: Serialization (err) => Some (err) , MessageError :: Io (err) => Some (err) , } } }
};
}
