// Generated macro for parse_const_arguments (function)
macro_rules! Depcrate_parseparse_const_arguments {
() => {
// Module: crate::parse
// Provides: {"parse_const_arguments"}
// Dependencies: {}
fn parse_const_arguments (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Vec < (Positioned < Name > , Positioned < ConstValue >) > > { debug_assert_eq ! (pair . as_rule () , Rule :: const_arguments) ; pair . into_inner () . map (| pair | { debug_assert_eq ! (pair . as_rule () , Rule :: const_argument) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let value = parse_const_value (pairs . next () . unwrap () , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok ((name , value)) }) . collect () }
};
}
