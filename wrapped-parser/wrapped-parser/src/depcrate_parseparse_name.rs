// Generated macro for parse_name (function)
macro_rules! Depcrate_parseparse_name {
() => {
// Module: crate::parse
// Provides: {"parse_name"}
// Dependencies: {}
fn parse_name (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Name > > { debug_assert_eq ! (pair . as_rule () , Rule :: name) ; Ok (Positioned :: new (Name :: new (pair . as_str ()) , pc . step (& pair))) }
};
}
