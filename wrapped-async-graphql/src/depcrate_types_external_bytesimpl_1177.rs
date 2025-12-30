// Generated macro for impl_1177 (impl)
macro_rules! Depcrate_types_external_bytesimpl_1177 {
() => {
// Module: crate::types::external::bytes
// Provides: {"impl_1177"}
// Dependencies: {}
# [doc = " The `Binary` scalar type represents binary data."] # [Scalar (internal)] impl ScalarType for Bytes { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Binary (data) => Ok (data) , _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Binary (_)) } fn to_value (& self) -> Value { Value :: Binary (self . clone ()) } }
};
}
