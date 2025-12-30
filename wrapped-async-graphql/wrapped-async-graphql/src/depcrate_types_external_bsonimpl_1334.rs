// Generated macro for impl_1334 (impl)
macro_rules! Depcrate_types_external_bsonimpl_1334 {
() => {
// Module: crate::types::external::bson
// Provides: {"impl_1334"}
// Dependencies: {}
# [Scalar (internal , name = "JSONObject")] impl ScalarType for Document { fn parse (value : Value) -> InputValueResult < Self > { bson :: to_document (& value) . map_err (InputValueError :: custom) } fn to_value (& self) -> Value { bson :: from_document (self . clone ()) . unwrap_or_default () } }
};
}
