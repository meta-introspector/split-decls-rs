// Generated macro for parse_boolean (function)
macro_rules! Depcrate_parseparse_boolean {
() => {
// Module: crate::parse
// Provides: {"parse_boolean"}
// Dependencies: {}
fn parse_boolean (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < bool > > { debug_assert_eq ! (pair . as_rule () , Rule :: boolean) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (match pair . as_str () { "true" => true , "false" => false , _ => unreachable ! () , } , pos ,)) }
};
}
