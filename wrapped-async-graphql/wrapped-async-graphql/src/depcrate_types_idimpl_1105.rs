// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_types_idimpl_1105 {
() => {
// Module: crate::types::id
// Provides: {"impl_1105"}
// Dependencies: {}
# [Scalar (internal , name = "ID")] impl ScalarType for ID { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) if n . is_i64 () => Ok (ID (n . to_string ())) , Value :: String (s) => Ok (ID (s)) , _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { match value { Value :: Number (n) if n . is_i64 () => true , Value :: String (_) => true , _ => false , } } fn to_value (& self) -> Value { Value :: String (self . 0 . clone ()) } }
};
}
