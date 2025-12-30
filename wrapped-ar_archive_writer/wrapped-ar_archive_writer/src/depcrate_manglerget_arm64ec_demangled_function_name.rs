// Generated macro for get_arm64ec_demangled_function_name (function)
macro_rules! Depcrate_manglerget_arm64ec_demangled_function_name {
() => {
// Module: crate::mangler
// Provides: {"get_arm64ec_demangled_function_name"}
// Dependencies: {}
pub fn get_arm64ec_demangled_function_name (name : & str) -> Option < String > { let first_char = name . chars () . next () . unwrap () ; if first_char == '#' { return Some (name [1 ..] . to_string ()) ; } if first_char != '?' { return None ; } match name . split_once ("$$h") { Some ((first , second)) if ! second . is_empty () => Some (format ! ("{first}{second}")) , _ => None , } }
};
}
