// Generated macro for get_arm64ec_mangled_function_name (function)
macro_rules! Depcrate_manglerget_arm64ec_mangled_function_name {
() => {
// Module: crate::mangler
// Provides: {"get_arm64ec_mangled_function_name"}
// Dependencies: {}
pub fn get_arm64ec_mangled_function_name (name : & str) -> Result < Option < String > , () > { let first_char = name . chars () . next () . unwrap () ; if first_char != '?' { if first_char == '#' { return Ok (None) ; } return Ok (Some (format ! ("#{name}"))) ; } if name . contains ("$$h") { return Ok (None) ; } Err (()) }
};
}
