// Generated macro for impl_1282 (impl)
macro_rules! Depcrate_types_external_non_zero_integersimpl_1282 {
() => {
// Module: crate::types::external::non_zero_integers
// Provides: {"impl_1282"}
// Dependencies: {}
# [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for NonZeroI8 { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_i64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n < i8 :: MIN as i64 || n > i8 :: MAX as i64 || n == 0 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} or non zero are accepted." , i8 :: MIN , i8 :: MAX))) ; } Ok (NonZeroI8 :: new (n as i8) . unwrap ()) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (self . get () as i64)) } }
};
}
