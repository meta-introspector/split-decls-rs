// Generated macro for impl_1360 (impl)
macro_rules! Depcrate_types_external_naive_timeimpl_1360 {
() => {
// Module: crate::types::external::naive_time
// Provides: {"impl_1360"}
// Dependencies: {}
# [Scalar (internal)] # [doc = " ISO 8601 time without timezone."] # [doc = " Allows for the nanosecond precision and optional leap second representation."] # [doc = " Format: %H:%M:%S%.f"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " * `08:59:60.123`"] impl ScalarType for NaiveTime { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (NaiveTime :: parse_from_str (& s , "%H:%M:%S%.f") ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . format ("%H:%M:%S%.f") . to_string ()) } }
};
}
