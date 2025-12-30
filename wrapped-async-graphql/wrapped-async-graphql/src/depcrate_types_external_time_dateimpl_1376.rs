// Generated macro for impl_1376 (impl)
macro_rules! Depcrate_types_external_time_dateimpl_1376 {
() => {
// Module: crate::types::external::time_date
// Provides: {"impl_1376"}
// Dependencies: {}
# [doc = " ISO 8601 calendar date without timezone."] # [doc = " Format: %Y-%m-%d"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " * `1994-11-13`"] # [doc = " * `2000-02-24`"] # [Scalar (internal , name = "Date")] impl ScalarType for Date { fn parse (value : Value) -> InputValueResult < Self > { match & value { Value :: String (s) => Ok (Self :: parse (s , & DATE_FORMAT) ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . format (& DATE_FORMAT) . unwrap_or_else (| e | panic ! ("Failed to format `Date`: {}" , e)) ,) } }
};
}
