// Generated macro for explain (function)
macro_rules! Depcrateexplain {
() => {
// Module: crate
// Provides: {"explain"}
// Dependencies: {}
pub fn explain (name : & str) -> i32 { let target = format ! ("clippy::{}" , name . to_ascii_uppercase ()) ; if let Some (info) = declared_lints :: LINTS . iter () . find (| info | info . lint . name == target) { println ! ("{}" , sanitize_explanation (info . explanation)) ; let mut mdconf = get_configuration_metadata () ; let name = name . to_ascii_lowercase () ; mdconf . retain (| cconf | cconf . lints . contains (& & * name)) ; if ! mdconf . is_empty () { println ! ("### Configuration for {}:\n" , info . lint . name_lower ()) ; for conf in mdconf { println ! ("{conf}") ; } } 0 } else { println ! ("unknown lint: {name}") ; 1 } }
};
}
