// Generated macro for impl_102 (impl)
macro_rules! Depcrate_serializationimpl_102 {
() => {
// Module: crate::serialization
// Provides: {"impl_102"}
// Dependencies: {}
impl Drop for SerializationSink { fn drop (& mut self) { let mut data = self . data . lock () ; let SerializationSinkInner { ref mut buffer , addr : _ , } = * data ; self . flush (buffer) ; } }
};
}
