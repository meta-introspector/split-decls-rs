// Generated macro for parse_string (function)
macro_rules! Depcrate_parseparse_string {
() => {
// Module: crate::parse
// Provides: {"parse_string"}
// Dependencies: {}
fn parse_string (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < String > > { debug_assert_eq ! (pair . as_rule () , Rule :: string) ; let pos = pc . step (& pair) ; let pair = exactly_one (pair . into_inner ()) ; Ok (Positioned :: new (match pair . as_rule () { Rule :: block_string_content => block_string_value (pair . as_str ()) , Rule :: string_content => string_value (pair . as_str ()) , _ => unreachable ! () , } , pos ,)) }
};
}
