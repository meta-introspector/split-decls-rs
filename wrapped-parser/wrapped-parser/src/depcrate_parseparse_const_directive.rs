// Generated macro for parse_const_directive (function)
macro_rules! Depcrate_parseparse_const_directive {
() => {
// Module: crate::parse
// Provides: {"parse_const_directive"}
// Dependencies: {}
fn parse_const_directive (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < ConstDirective > > { debug_assert_eq ! (pair . as_rule () , Rule :: const_directive) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let arguments = parse_if_rule (& mut pairs , Rule :: const_arguments , | pair | { parse_const_arguments (pair , pc) }) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (ConstDirective { name , arguments : arguments . unwrap_or_default () , } , pos ,)) }
};
}
