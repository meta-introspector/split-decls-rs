// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_types_external_non_zero_integersimpl_1291 {
() => {
// Module: crate::types::external::non_zero_integers
// Provides: {"impl_1291"}
// Dependencies: {}
# [doc = " The `Int` scalar type represents non-fractional whole numeric values."] # [Scalar (internal , name = "Int")] impl ScalarType for NonZeroUsize { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: Number (n) => { let n = n . as_u64 () . ok_or_else (| | InputValueError :: from ("Invalid number")) ? ; if n > usize :: MAX as u64 || n == 0 { return Err (InputValueError :: from (format ! ("Only integers from {} to {} or non zero are accepted." , 1 , usize :: MAX))) ; } Ok (NonZeroUsize :: new (n as usize) . unwrap ()) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: Number (n) if n . is_i64 ()) } fn to_value (& self) -> Value { Value :: Number (Number :: from (self . get () as u64)) } }
};
}
