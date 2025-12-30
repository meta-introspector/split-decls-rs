// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_types_external_boolimpl_1173 {
() => {
// Module: crate::types::external::bool
// Provides: {"impl_1173"}
// Dependencies: {}
# [doc = " The `Boolean` scalar type represents `true` or `false`."] # [Scalar (internal , name = "Boolean")] impl ScalarType for bool { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Boolean (n) => Ok (n) , _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Boolean (_)) } fn to_value (& self) -> Value { Value :: Boolean (* self) } }
};
}
