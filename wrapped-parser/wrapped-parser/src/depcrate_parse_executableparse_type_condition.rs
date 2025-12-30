// Generated macro for parse_type_condition (function)
macro_rules! Depcrate_parse_executableparse_type_condition {
() => {
// Module: crate::parse::executable
// Provides: {"parse_type_condition"}
// Dependencies: {}
fn parse_type_condition (pair : Pair < Rule > , pc : & mut PositionCalculator ,) -> Result < Positioned < TypeCondition > > { debug_assert_eq ! (pair . as_rule () , Rule :: type_condition) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (TypeCondition { on : parse_name (exactly_one (pair . into_inner ()) , pc) ? , } , pos ,)) }
};
}
