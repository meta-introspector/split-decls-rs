// Generated macro for parse_definition_item (function)
macro_rules! Depcrate_parse_executableparse_definition_item {
() => {
// Module: crate::parse::executable
// Provides: {"parse_definition_item"}
// Dependencies: {}
fn parse_definition_item (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < DefinitionItem > { debug_assert_eq ! (pair . as_rule () , Rule :: executable_definition) ; let pair = exactly_one (pair . into_inner ()) ; Ok (match pair . as_rule () { Rule :: operation_definition => { DefinitionItem :: Operation (parse_operation_definition_item (pair , pc) ?) } Rule :: fragment_definition => { DefinitionItem :: Fragment (parse_fragment_definition_item (pair , pc) ?) } _ => unreachable ! () , }) }
};
}
