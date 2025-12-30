// Generated macro for parse_enum_value (function)
macro_rules! Depcrate_parseparse_enum_value {
() => {
// Module: crate::parse
// Provides: {"parse_enum_value"}
// Dependencies: {}
fn parse_enum_value (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: enum_value) ; parse_name (exactly_one (pair . into_inner ()) , pc) }
};
}
