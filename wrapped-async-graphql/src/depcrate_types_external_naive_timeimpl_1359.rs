// Generated macro for impl_1359 (impl)
macro_rules! Depcrate_types_external_naive_timeimpl_1359 {
() => {
// Module: crate::types::external::naive_time
// Provides: {"impl_1359"}
// Dependencies: {}
# [Scalar (internal)] # [doc = " ISO 8601 calendar date without timezone."] # [doc = " Format: %Y-%m-%d"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " * `1994-11-13`"] # [doc = " * `2000-02-24`"] impl ScalarType for NaiveDate { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (NaiveDate :: parse_from_str (& s , "%Y-%m-%d") ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . format ("%Y-%m-%d") . to_string ()) } }
};
}
