// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_types_string_numberimpl_1158 {
() => {
// Module: crate::types::string_number
// Provides: {"impl_1158"}
// Dependencies: {}
# [Scalar (internal)] impl < T : Num + Display + Send + Sync > ScalarType for StringNumber < T > where < T as Num > :: FromStrRadixErr : Display , { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => { let n = T :: from_str_radix (& s , 10) . map_err (| err | InputValueError :: custom (err . to_string ())) ? ; Ok (StringNumber (n)) } _ => Err (InputValueError :: expected_type (value)) , } } fn is_valid (value : & Value) -> bool { matches ! (value , Value :: String (_)) } fn to_value (& self) -> Value { Value :: String (self . 0 . to_string ()) } }
};
}
