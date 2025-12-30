// Generated macro for serialize_with_options (function)
macro_rules! Depcrate_serializerserialize_with_options {
() => {
// Module: crate::serializer
// Provides: {"serialize_with_options"}
// Dependencies: {}
# [doc = " Serializes an abstract syntax tree representing a Fluent Translation List into a"] # [doc = " String accepting custom options."] pub fn serialize_with_options < 's , S : Slice < 's > > (resource : & Resource < S > , options : Options ,) -> String { let mut ser = Serializer :: new (options) ; ser . serialize_resource (resource) ; ser . into_serialized_text () }
};
}
