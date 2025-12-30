// Generated macro for parse_number (function)
macro_rules! Depcrate_parseparse_number {
() => {
// Module: crate::parse
// Provides: {"parse_number"}
// Dependencies: {}
fn parse_number (pair : Pair < Rule > , pc : & mut PositionCalculator) -> Result < Positioned < Number > > { debug_assert_eq ! (pair . as_rule () , Rule :: number) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (pair . as_str () . parse () . map_err (| err | Error :: Syntax { message : format ! ("invalid number: {}" , err) , start : pos , end : None , }) ? , pos ,)) }
};
}
