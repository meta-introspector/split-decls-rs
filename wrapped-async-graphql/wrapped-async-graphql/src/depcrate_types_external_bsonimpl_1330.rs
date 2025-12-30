// Generated macro for impl_1330 (impl)
macro_rules! Depcrate_types_external_bsonimpl_1330 {
() => {
// Module: crate::types::external::bson
// Provides: {"impl_1330"}
// Dependencies: {}
# [Scalar (internal)] impl ScalarType for ObjectId { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (ObjectId :: parse_str (s) ?) , Value :: Object (o) => { let json = Value :: Object (o) . into_json () ? ; let bson = Bson :: try_from (json) ? ; bson . as_object_id () . ok_or_else (| | { InputValueError :: custom ("could not parse the value as a BSON ObjectId") }) } _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
};
}
