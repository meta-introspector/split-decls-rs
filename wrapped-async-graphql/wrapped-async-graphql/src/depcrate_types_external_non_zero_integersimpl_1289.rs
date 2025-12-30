// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_types_external_non_zero_integersimpl_1289 {
() => {
// Module: crate::types::external::non_zero_integers
// Provides: {"impl_1289"}
// Dependencies: {}
# [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for NonZeroU32 { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_u64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n > u32 :: MAX as u64 || n == 0 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} or non zero are accepted." , 1 , u32 :: MAX))) ; } Ok (NonZeroU32 :: new (n as u32) . unwrap ()) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (self . get () as u64)) } }
};
}
