// Generated macro for impl_1200 (impl)
macro_rules! Depcrate_types_external_integersimpl_1200 {
() => {
// Module: crate::types::external::integers
// Provides: {"impl_1200"}
// Dependencies: {}
# [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for u64 { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_u64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; Ok (n as Self) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_u64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (* self)) } }
};
}
