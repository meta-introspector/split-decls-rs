// Generated macro for tests (module)
macro_rules! Depcrate_types_external_bsontests {
() => {
// Module: crate::types::external::bson
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use serde_json :: json ; use super :: * ; # [test] fn test_parse_bson_uuid () { let id = Uuid :: new () ; let bson_value = bson :: bson ! (id) ; let extended_json_value = json ! (bson_value) ; let gql_value = Value :: from_json (extended_json_value) . expect ("valid json") ; assert_eq ! (id , < Uuid as ScalarType >:: parse (gql_value) . expect ("parsing succeeds")) ; } # [test] fn test_parse_bson_object_id () { let id = ObjectId :: from_bytes ([42 ; 12]) ; let bson_value = bson :: bson ! (id) ; let extended_json_value = json ! (bson_value) ; let gql_value = Value :: from_json (extended_json_value) . expect ("valid json") ; assert_eq ! (id , < ObjectId as ScalarType >:: parse (gql_value) . expect ("parsing succeeds")) ; } }
};
}
