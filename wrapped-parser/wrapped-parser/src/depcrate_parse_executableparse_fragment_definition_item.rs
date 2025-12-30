// Generated macro for parse_fragment_definition_item (function)
macro_rules! Depcrate_parse_executableparse_fragment_definition_item {
() => {
// Module: crate::parse::executable
// Provides: {"parse_fragment_definition_item"}
// Dependencies: {}
fn parse_fragment_definition_item (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < FragmentDefinitionItem > > { debug_assert_eq ! (pair . as_rule () , Rule :: fragment_definition) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let type_condition = parse_type_condition (pairs . next () . unwrap () , pc) ? ; let directives = parse_opt_directives (& mut pairs , pc) ? ; let selection_set = parse_selection_set (pairs . next () . unwrap () , pc , MAX_RECURSION_DEPTH) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (FragmentDefinitionItem { name , definition : FragmentDefinition { type_condition , directives , selection_set , } , } , pos ,)) }
};
}
