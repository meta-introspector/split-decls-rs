// Generated macro for impl_1333 (impl)
macro_rules! Depcrate_types_external_bsonimpl_1333 {
() => {
// Module: crate::types::external::bson
// Provides: {"impl_1333"}
// Dependencies: {}
# [Scalar (internal , name = "JSON")] impl ScalarType for Bson { fn parse (value : Value) -> InputValueResult < Self > { bson :: to_bson (& value) . map_err (InputValueError :: custom) } fn to_value (& self) -> Value { bson :: from_bson (self . clone ()) . unwrap_or_default () } }
};
}
