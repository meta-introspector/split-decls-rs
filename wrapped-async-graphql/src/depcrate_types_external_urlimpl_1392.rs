// Generated macro for impl_1392 (impl)
macro_rules! Depcrate_types_external_urlimpl_1392 {
() => {
// Module: crate::types::external::url
// Provides: {"impl_1392"}
// Dependencies: {}
# [Scalar (internal , specified_by_url = "http://url.spec.whatwg.org/")] # [doc = " URL is a String implementing the [URL Standard](http://url.spec.whatwg.org/)"] impl ScalarType for Url { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (Url :: parse (& s) ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
};
}
