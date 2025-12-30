// Generated macro for impl_1396 (impl)
macro_rules! Depcrate_types_external_uuidimpl_1396 {
() => {
// Module: crate::types::external::uuid
// Provides: {"impl_1396"}
// Dependencies: {}
# [Scalar (internal , name = "UUID" , specified_by_url = "http://tools.ietf.org/html/rfc4122")] # [doc = " A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as"] # [doc = " Strings within GraphQL. UUIDs are used to assign unique identifiers to"] # [doc = " entities without requiring a central allocating authority."] # [doc = ""] # [doc = " # References"] # [doc = ""] # [doc = " * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)"] # [doc = " * [RFC4122: A Universally Unique Identifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)"] impl ScalarType for Uuid { fn parse (value : Value) -> InputValueResult < Self > { match value { Value :: String (s) => Ok (Uuid :: parse_str (& s) ?) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } }
};
}
