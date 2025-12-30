// Generated macro for parse_variable (function)
macro_rules! Depcrate_parseparse_variable {
() => {
// Module: crate::parse
// Provides: {"parse_variable"}
// Dependencies: {}
fn parse_variable (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: variable) ; parse_name (exactly_one (pair . into_inner ()) , pc) }
};
}
