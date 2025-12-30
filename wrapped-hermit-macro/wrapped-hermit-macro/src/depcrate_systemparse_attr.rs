// Generated macro for parse_attr (function)
macro_rules! Depcrate_systemparse_attr {
() => {
// Module: crate::system
// Provides: {"parse_attr"}
// Dependencies: {}
fn parse_attr (attr : Option < Ident >) -> Result < bool > { let Some (label) = attr else { return Ok (false) ; } ; if label == "errno" { return Ok (true) ; } bail ! (label , "#[system] can only contain \"errno\" or nothing") ; }
};
}
