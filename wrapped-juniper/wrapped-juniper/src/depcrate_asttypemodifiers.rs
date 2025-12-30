// Generated macro for TypeModifiers (enum)
macro_rules! Depcrate_astTypeModifiers {
() => {
// Module: crate::ast
// Provides: {"TypeModifiers"}
// Dependencies: {}
# [doc = " Owned slice of [`TypeModifier`]s."] # [derive (Clone , Debug)] pub enum TypeModifiers { # [doc = " [`TypeModifier`]s known statically."] Static (& 'static [TypeModifier]) , # [doc = " [`TypeModifier`]s built dynamically."] Dynamic (Box < [TypeModifier] >) , }
};
}
