// Generated macro for test (module)
macro_rules! Depcrate_integrations_bsontest {
() => {
// Module: crate::integrations::bson
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use bson :: oid :: ObjectId ; use crate :: { FromInputValue , InputValue , graphql } ; # [test] fn objectid_from_input () { let raw = "53e37d08776f724e42000000" ; let input : InputValue = graphql :: input_value ! ((raw)) ; let parsed : ObjectId = FromInputValue :: from_input_value (& input) . unwrap () ; let id = ObjectId :: parse_str (raw) . unwrap () ; assert_eq ! (parsed , id) ; } }
};
}
