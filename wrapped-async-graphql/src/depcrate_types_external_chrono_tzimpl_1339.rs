// Generated macro for impl_1339 (impl)
macro_rules! Depcrate_types_external_chrono_tzimpl_1339 {
() => {
// Module: crate::types::external::chrono_tz
// Provides: {"impl_1339"}
// Dependencies: {}
# [Scalar (internal , name = "TimeZone" , specified_by_url = "http://www.iana.org/time-zones")] impl ScalarType for Tz { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (s . parse () ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . name () . to_owned ()) } }
};
}
