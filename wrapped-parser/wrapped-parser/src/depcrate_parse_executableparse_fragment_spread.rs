// Generated macro for parse_fragment_spread (function)
macro_rules! Depcrate_parse_executableparse_fragment_spread {
() => {
// Module: crate::parse::executable
// Provides: {"parse_fragment_spread"}
// Dependencies: {}
fn parse_fragment_spread (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < FragmentSpread > > { debug_assert_eq ! (pair . as_rule () , Rule :: fragment_spread) ; let pos = pc . step (& pair) ; let mut pairs = pair . into_inner () ; let fragment_name = parse_name (pairs . next () . unwrap () , pc) ? ; let directives = parse_opt_directives (& mut pairs , pc) ? ; debug_assert_eq ! (pairs . next () , None) ; Ok (Positioned :: new (FragmentSpread { fragment_name , directives , } , pos ,)) }
};
}
