// Generated macro for serialize_value (function)
macro_rules! Depcrate_serializationserialize_value {
() => {
// Module: crate::serialization
// Provides: {"serialize_value"}
// Dependencies: {}
pub fn serialize_value < S : Serialize > (s : & S , format : SerializationFormat) -> String { let serializer = ContentSerializer :: < ValueError > :: new () ; let content = Serialize :: serialize (s , serializer) . unwrap () ; serialize_content (content , format) }
};
}
