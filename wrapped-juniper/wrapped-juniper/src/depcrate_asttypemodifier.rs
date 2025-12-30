// Generated macro for TypeModifier (enum)
macro_rules! Depcrate_astTypeModifier {
() => {
// Module: crate::ast
// Provides: {"TypeModifier"}
// Dependencies: {}
# [doc = " Possible modifiers in a [`Type`] literal."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum TypeModifier { # [doc = " Non-`null` type (e.g. `<type>!`)."] NonNull , # [doc = " List of types (e.g. `[<type>]`)."] List (Option < usize >) , }
};
}
