// Generated macro for parse_type (function)
macro_rules! Depcrate_parseparse_type {
() => {
// Module: crate::parse
// Provides: {"parse_type"}
// Dependencies: {}
fn parse_type (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Type > > { debug_assert_eq ! (pair . as_rule () , Rule :: type_) ; Ok (Positioned :: new (Type :: new (pair . as_str ()) . unwrap () , pc . step (& pair) ,)) }
};
}
