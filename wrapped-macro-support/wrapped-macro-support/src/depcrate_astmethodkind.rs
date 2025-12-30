// Generated macro for MethodKind (enum)
macro_rules! Depcrate_astMethodKind {
() => {
// Module: crate::ast
// Provides: {"MethodKind"}
// Dependencies: {}
# [doc = " The type of a method"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub enum MethodKind { # [doc = " A class constructor"] Constructor , # [doc = " Any other kind of method"] Operation (Operation) , }
};
}
