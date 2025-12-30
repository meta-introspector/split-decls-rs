// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_types_external_datetimeimpl_1344 {
() => {
// Module: crate::types::external::datetime
// Provides: {"impl_1344"}
// Dependencies: {}
# [doc = " Implement the DateTime<Local> scalar"] # [doc = ""] # [doc = " The input/output is a string in RFC3339 format."] # [Scalar (internal , name = "DateTime" , specified_by_url = "https://datatracker.ietf.org/doc/html/rfc3339")] impl ScalarType for DateTime < Local > { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (s . parse :: < DateTime < Local > > () ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_rfc3339 ()) } }
};
}
