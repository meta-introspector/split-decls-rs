// Generated macro for parse_string (function)
macro_rules! Depcrate_deparse_string {
() => {
// Module: crate::de
// Provides: {"parse_string"}
// Dependencies: {}
fn parse_string (pair : Pair < '_ , Rule >) -> Result < String > { let span = pair . as_span () ; let mut res = parse_string_component (pair) ; error :: set_location (& mut res , & span) ; res }
};
}
