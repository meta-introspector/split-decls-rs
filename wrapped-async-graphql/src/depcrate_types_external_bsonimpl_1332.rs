// Generated macro for impl_1332 (impl)
macro_rules! Depcrate_types_external_bsonimpl_1332 {
() => {
// Module: crate::types::external::bson
// Provides: {"impl_1332"}
// Dependencies: {}
# [cfg (feature = "chrono")] # [Scalar (internal , name = "DateTime")] impl ScalarType for UtcDateTime { fn parse (value : Value) -> InputValueResult < Self > { < DateTime < Utc > > :: parse (value) . map_err (InputValueError :: propagate) . map (UtcDateTime :: from_chrono) } fn to_value (& self) -> Value { self . to_chrono () . to_value () } }
};
}
