// Generated macro for test (module)
macro_rules! Depcrate_integrations_uuidtest {
() => {
// Module: crate::integrations::uuid
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use uuid :: Uuid ; use crate :: { FromInputValue , InputValue , graphql } ; # [test] fn uuid_from_input () { let raw = "123e4567-e89b-12d3-a456-426655440000" ; let input : InputValue = graphql :: input_value ! ((raw)) ; let parsed : Uuid = FromInputValue :: from_input_value (& input) . unwrap () ; let id = Uuid :: parse_str (raw) . unwrap () ; assert_eq ! (parsed , id) ; } }
};
}
