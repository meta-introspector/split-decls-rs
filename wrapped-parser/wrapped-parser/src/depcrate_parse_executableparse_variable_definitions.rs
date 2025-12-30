// Generated macro for parse_variable_definitions (function)
macro_rules! Depcrate_parse_executableparse_variable_definitions {
() => {
// Module: crate::parse::executable
// Provides: {"parse_variable_definitions"}
// Dependencies: {}
fn parse_variable_definitions (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < Positioned < VariableDefinition > > > { debug_assert_eq ! (pair . as_rule () , Rule :: variable_definitions) ; pair . into_inner () . map (| pair | parse_variable_definition (pair , pc)) . collect () }
};
}
