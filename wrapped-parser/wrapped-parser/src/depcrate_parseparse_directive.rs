// Generated macro for parse_directive (function)
macro_rules! Depcrate_parseparse_directive {
() => {
// Module: crate::parse
// Provides: {"parse_directive"}
// Dependencies: {}
fn parse_directive (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Directive > > { debug_assert_eq ! (pair . as_rule () , Rule :: directive) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let name = parse_name (pairs . next () . unwrap () , pc) ? ; let arguments = parse_if_rule (& mut pairs , Rule :: arguments , | pair | { parse_arguments (pair , pc) }) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (Directive { name , arguments : arguments . unwrap_or_default () , } , pos ,)) }
};
}
