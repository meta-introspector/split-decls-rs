// Generated macro for gen_to_possible_value (function)
macro_rules! Depcrate_derives_value_enumgen_to_possible_value {
() => {
// Module: crate::derives::value_enum
// Provides: {"gen_to_possible_value"}
// Dependencies: {}
fn gen_to_possible_value (item : & Item , lits : & [(TokenStream , Ident)]) -> TokenStream { let (lit , variant) : (Vec < TokenStream > , Vec < Ident >) = lits . iter () . cloned () . unzip () ; let deprecations = item . deprecations () ; quote ! { fn to_possible_value <'a > (& self) -> :: std :: option :: Option < clap :: builder :: PossibleValue > { # deprecations match self { # (Self ::# variant => Some (# lit) ,) * _ => None } } } }
};
}
