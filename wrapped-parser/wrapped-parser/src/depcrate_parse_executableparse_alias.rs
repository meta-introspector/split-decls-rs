// Generated macro for parse_alias (function)
macro_rules! Depcrate_parse_executableparse_alias {
() => {
// Module: crate::parse::executable
// Provides: {"parse_alias"}
// Dependencies: {}
fn parse_alias (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: alias) ; parse_name (exactly_one (pair . into_inner ()) , pc) }
};
}
