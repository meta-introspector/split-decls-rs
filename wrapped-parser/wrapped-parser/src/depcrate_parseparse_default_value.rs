// Generated macro for parse_default_value (function)
macro_rules! Depcrate_parseparse_default_value {
() => {
// Module: crate::parse
// Provides: {"parse_default_value"}
// Dependencies: {}
fn parse_default_value (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < ConstValue > > { debug_assert_eq ! (pair . as_rule () , Rule :: default_value) ; parse_const_value (exactly_one (pair . into_inner ()) , pc) }
};
}
