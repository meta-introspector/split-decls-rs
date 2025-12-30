// Generated macro for build (function)
macro_rules! Depcrate_arbitrary_in_macro_rulesbuild {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"build"}
// Dependencies: {}
pub fn build () -> Result < () > { let s = std :: fs :: read_to_string ("tests/arbitrary.rs") ? ; let mut file : File = parse_str (& s) ? ; transform (& mut file) ? ; let s = file . to_token_stream () . to_string () ; let s = rustfmt (& s) ; std :: fs :: write ("tests/arbitrary_in_macro_rules.rs" , s) ? ; Ok (()) }
};
}
