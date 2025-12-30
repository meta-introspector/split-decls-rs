// Generated macro for parse_definition_items (function)
macro_rules! Depcrate_parse_executableparse_definition_items {
() => {
// Module: crate::parse::executable
// Provides: {"parse_definition_items"}
// Dependencies: {}
fn parse_definition_items (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < DefinitionItem > > { debug_assert_eq ! (pair . as_rule () , Rule :: executable_document) ; Ok (pair . into_inner () . filter (| pair | pair . as_rule () != Rule :: EOI) . map (| pair | parse_definition_item (pair , pc)) . collect :: < Result < _ > > () ?) }
};
}
