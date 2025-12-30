// Generated macro for parse_arguments_definition (function)
macro_rules! Depcrate_parse_serviceparse_arguments_definition {
() => {
// Module: crate::parse::service
// Provides: {"parse_arguments_definition"}
// Dependencies: {}
fn parse_arguments_definition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < InputValueDefinition > > > { debug_assert_eq ! (pair . as_rule () , Rule :: arguments_definition) ; pair . into_inner () . map (| pair | parse_input_value_definition (pair , pc)) . collect () }
};
}
