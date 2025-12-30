// Generated macro for lit_string_value (function)
macro_rules! Depcrate_methods_no_effect_replacelit_string_value {
() => {
// Module: crate::methods::no_effect_replace
// Provides: {"lit_string_value"}
// Dependencies: {}
fn lit_string_value (node : & LitKind) -> Option < String > { match node { LitKind :: Char (value) => Some (value . to_string ()) , LitKind :: Str (value , _) => Some (value . as_str () . to_owned ()) , _ => None , } }
};
}
