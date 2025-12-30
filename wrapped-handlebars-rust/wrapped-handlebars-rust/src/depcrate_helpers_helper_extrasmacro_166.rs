// Generated macro for macro_166 (macro)
macro_rules! Depcrate_helpers_helper_extrasmacro_166 {
() => {
// Module: crate::helpers::helper_extras
// Provides: {"macro_166"}
// Dependencies: {}
handlebars_helper ! (len : | x : Json | { match x { Json :: Array (a) => a . len () , Json :: Object (m) => m . len () , Json :: String (s) => s . len () , _ => 0 } }) ;
};
}
