// Generated macro for generate_macro_name (function)
macro_rules! Depcrate_arbitrary_in_macro_rulesgenerate_macro_name {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"generate_macro_name"}
// Dependencies: {}
fn generate_macro_name (traits : & [String]) -> String { let mut name = "macro_rules" . to_string () ; for trait_name in traits { name . push ('_') ; name . push_str (& trait_name . to_lowercase ()) ; } name }
};
}
