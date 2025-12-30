// Generated macro for FlagsItemKind (enum)
macro_rules! Depcrate_astFlagsItemKind {
() => {
// Module: crate::ast
// Provides: {"FlagsItemKind"}
// Dependencies: {}
# [doc = " The kind of an item in a group of flags."] # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub enum FlagsItemKind { # [doc = " A negation operator applied to all subsequent flags in the enclosing"] # [doc = " group."] Negation , # [doc = " A single flag in a group."] Flag (Flag) , }
};
}
